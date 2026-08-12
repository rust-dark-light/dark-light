use std::sync::mpsc;
use std::thread::JoinHandle;

use crate::{Error, Mode, Watcher};
use zbus::proxy;
use zbus::zvariant::OwnedValue;

const APPEARANCE: &str = "org.freedesktop.appearance";
const COLOR_SCHEME: &str = "color-scheme";
const PORTAL_DESTINATION: &str = "org.freedesktop.portal.Desktop";

#[proxy(
    interface = "org.freedesktop.portal.Settings",
    default_path = "/org/freedesktop/portal/desktop"
)]
pub trait XdgPortalSettings {
    fn read_one(&self, namespace: &str, key: &str) -> zbus::Result<OwnedValue>;

    #[zbus(signal)]
    fn setting_changed(&self, namespace: &str, key: &str, value: OwnedValue) -> zbus::Result<()>;
}

pub fn detect() -> Result<Mode, Error> {
    let conn = zbus::blocking::Connection::session()
        .map_err(|e| Error::XdgDesktopPortal(e.to_string()))?;
    let portal = XdgPortalSettingsProxyBlocking::new(&conn, PORTAL_DESTINATION)
        .map_err(|e| Error::XdgDesktopPortal(e.to_string()))?;
    let mode_value: u32 = portal
        .read_one(APPEARANCE, COLOR_SCHEME)
        .map_err(|e| Error::XdgDesktopPortal(e.to_string()))?
        .try_into()
        .map_err(|_| Error::XdgDesktopPortal("type convert failed".to_string()))?;
    Ok(match mode_value {
        1 => Mode::Dark,
        2 => Mode::Light,
        _ => Mode::Unspecified,
    })
}

/// Background handle for a Linux/BSD theme watcher.
///
/// The watcher thread blocks on the D-Bus session connection's signal stream, which has
/// no external cancellation mechanism; it is detached rather than joined on drop and will
/// exit the next time it observes that the channel receiver has been dropped.
pub struct WatchGuard(#[allow(dead_code)] Option<JoinHandle<()>>);

pub fn subscribe() -> Result<Watcher, Error> {
    let (sender, receiver) = mpsc::channel();

    let conn = zbus::blocking::Connection::session()
        .map_err(|e| Error::XdgDesktopPortal(e.to_string()))?;
    let portal = XdgPortalSettingsProxyBlocking::new(&conn, PORTAL_DESTINATION)
        .map_err(|e| Error::XdgDesktopPortal(e.to_string()))?;
    let changes = portal
        .receive_setting_changed()
        .map_err(|e| Error::XdgDesktopPortal(e.to_string()))?;

    let handle = std::thread::spawn(move || {
        let mut last = None;
        for signal in changes {
            let Ok(args) = signal.args() else { continue };
            if args.namespace != APPEARANCE || args.key != COLOR_SCHEME {
                continue;
            }
            let Ok(value): Result<u32, _> = args.value.try_into() else {
                continue;
            };
            let mode = match value {
                1 => Mode::Dark,
                2 => Mode::Light,
                _ => Mode::Unspecified,
            };
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
        guard: WatchGuard(Some(handle)),
    })
}
