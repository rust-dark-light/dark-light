// Dark/light mode detection on macOS.
// Written with help from Ryan McGrath (https://rymc.io/).

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Duration;

use crate::{Error, Mode, Watcher};
use objc2_foundation::{ns_string, NSString, NSUserDefaults};

pub fn detect() -> Result<Mode, Error> {
    let style = NSUserDefaults::standardUserDefaults()
        .persistentDomainForName(ns_string!("Apple Global Domain"))
        .ok_or(Error::PersistentDomainFailed)?
        .objectForKey(ns_string!("AppleInterfaceStyle"));

    let Some(style) = style else {
        return Ok(Mode::Light);
    };

    let Ok(style) = style.downcast::<NSString>() else {
        return Err(Error::PersistentDomainFailed);
    };
    let mode = style.isEqualToString(ns_string!("Dark")).into();
    Ok(mode)
}

const POLL_INTERVAL: Duration = Duration::from_secs(1);

/// Background handle for a macOS theme watcher.
///
/// macOS theme changes are observed by polling [`detect`] on an interval, since
/// registering for `NSDistributedNotificationCenter`'s
/// `AppleInterfaceThemeChangedNotification` requires an Objective-C delegate object and a
/// live `NSRunLoop`, which the objc2 bindings currently in use don't expose a safe,
/// block-based path for.
pub struct WatchGuard {
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl Drop for WatchGuard {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

pub fn subscribe() -> Result<Watcher, Error> {
    let (sender, receiver) = mpsc::channel();
    let stop = Arc::new(AtomicBool::new(false));

    let mut last = detect()?;
    let watcher_stop = Arc::clone(&stop);
    let handle = std::thread::spawn(move || {
        while !watcher_stop.load(Ordering::Relaxed) {
            std::thread::sleep(POLL_INTERVAL);
            let Ok(mode) = detect() else { continue };
            if mode == last {
                continue;
            }
            last = mode;
            if sender.send(mode).is_err() {
                break;
            }
        }
    });

    Ok(Watcher {
        receiver,
        guard: WatchGuard {
            stop,
            handle: Some(handle),
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_returns_mode() {
        let result = detect();
        assert!(result.is_ok(), "detect() should return a valid mode");
        let mode = result.unwrap();
        if !matches!(mode, Mode::Light | Mode::Dark) {
            eprintln!("Warning: Unexpected mode value: {:?}", mode);
        }
    }
}
