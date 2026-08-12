use std::sync::mpsc;
use std::thread::JoinHandle;

use crate::{Error, Mode, Watcher};
use windows_sys::Win32::System::Registry::{RegNotifyChangeKeyValue, REG_NOTIFY_CHANGE_LAST_SET};
use winreg::enums::{HKEY_CURRENT_USER, KEY_NOTIFY, KEY_READ};
use winreg::{RegKey, HKEY};

const SUBKEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize";
const VALUE: &str = "AppsUseLightTheme";

pub fn detect() -> Result<Mode, Error> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let subkey = hkcu.open_subkey(SUBKEY)?;
    let dword: u32 = subkey.get_value(VALUE)?;
    Ok((dword == 0).into())
}

/// Background handle for a Windows theme watcher.
pub struct WatchGuard {
    hkey: HKEY,
    handle: Option<JoinHandle<()>>,
}

// `HKEY` is an opaque registry handle value; it is safe to hand off to the
// watcher thread, which becomes its sole owner until the thread exits.
unsafe impl Send for WatchGuard {}

impl Drop for WatchGuard {
    fn drop(&mut self) {
        // Closing the handle unblocks a pending `RegNotifyChangeKeyValue` call, letting
        // the watcher thread observe the error and exit so we can join it.
        unsafe { windows_sys::Win32::System::Registry::RegCloseKey(self.hkey) };
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

// `HKEY` is an opaque registry handle value with no thread affinity; the Win32
// registry API is safe to call from any thread.
struct SendHkey(HKEY);
unsafe impl Send for SendHkey {}

pub fn subscribe() -> Result<Watcher, Error> {
    let (sender, receiver) = mpsc::channel();

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu.open_subkey_with_flags(SUBKEY, KEY_READ | KEY_NOTIFY)?;
    let raw_hkey = key.raw_handle();
    // The handle now lives on for the watcher thread; `WatchGuard::drop` closes it.
    std::mem::forget(key);
    let send_hkey = SendHkey(raw_hkey);

    let handle = std::thread::spawn(move || {
        let send_hkey = send_hkey;
        let hkey = send_hkey.0;
        let mut last = None;
        loop {
            let status = unsafe {
                RegNotifyChangeKeyValue(
                    hkey,
                    0,
                    REG_NOTIFY_CHANGE_LAST_SET,
                    std::ptr::null_mut(),
                    0,
                )
            };
            if status != 0 {
                break;
            }
            // Re-open a fresh handle to read the value: the notify handle above isn't a
            // `winreg::RegKey`, so it has no typed read API of its own.
            let dword: u32 = match RegKey::predef(HKEY_CURRENT_USER)
                .open_subkey(SUBKEY)
                .and_then(|k| k.get_value(VALUE))
            {
                Ok(v) => v,
                Err(_) => break,
            };
            let mode: Mode = (dword == 0).into();
            if Some(mode) == last {
                continue;
            }
            last = Some(mode);
            if sender.send(mode).is_err() {
                break;
            }
        }
    });

    Ok(Watcher {
        receiver,
        guard: WatchGuard {
            hkey: raw_hkey,
            handle: Some(handle),
        },
    })
}
