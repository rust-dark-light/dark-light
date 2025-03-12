use crate::{Error, Mode};
use zbus::proxy;
use zbus::zvariant::OwnedValue;

const APPEARANCE: &str = "org.freedesktop.appearance";
const COLOR_SCHEME: &str = "color-scheme";
const PROTAL_DESTINATION: &str = "org.freedesktop.portal.Desktop";

#[proxy(
    interface = "org.freedesktop.portal.Settings",
    default_path = "/org/freedesktop/portal/desktop"
)]
pub trait XdgProtalSettings {
    fn read_one(&self, namespace: &str, key: &str) -> zbus::Result<OwnedValue>;
}
pub fn detect() -> Result<Mode, Error> {
    let conn = zbus::blocking::Connection::session()
        .map_err(|e| Error::XdgDesktopPortal(e.to_string()))?;
    let portal = XdgProtalSettingsProxyBlocking::new(&conn, PROTAL_DESTINATION)
        .map_err(|e| Error::XdgDesktopPortal(e.to_string()))?;
    let settings: u32 = portal
        .read_one(APPEARANCE, COLOR_SCHEME)
        .map_err(|e| Error::XdgDesktopPortal(e.to_string()))?
        .try_into()
        .map_err(|_| Error::XdgDesktopPortal("type convert failed".to_string()))?;
    Ok(settings.into())
}

impl From<u32> for Mode {
    fn from(value: u32) -> Self {
        match value {
            1 => Mode::Dark,
            2 => Mode::Light,
            _ => Mode::Unspecified,
        }
    }
}
