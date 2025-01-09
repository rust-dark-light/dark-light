use std::time::Duration;

use crate::{Error, Mode};

use ashpd::desktop::settings::ColorScheme as PortalColorScheme;
use ashpd::desktop::settings::Settings as XdgPortalSettings;
use async_std::{future, task};

pub fn detect() -> Result<Mode, Error> {
    task::block_on(future::timeout(Duration::from_millis(25), async {
        let settings = XdgPortalSettings::new()
            .await
            .map_err(|e| Error::XdgDesktopPortal(e.to_string()))?;
        let color_scheme = settings
            .color_scheme()
            .await
            .map_err(|e| Error::XdgDesktopPortal(e.to_string()))?;
        Ok::<Mode, Error>(color_scheme.into())
    }))
    .map_err(|_| Error::Timeout)?
}

impl From<PortalColorScheme> for Mode {
    fn from(value: PortalColorScheme) -> Self {
        match value {
            PortalColorScheme::NoPreference => Mode::Unspecified,
            PortalColorScheme::PreferDark => Mode::Dark,
            PortalColorScheme::PreferLight => Mode::Light,
        }
    }
}
