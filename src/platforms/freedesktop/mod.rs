pub mod detect;
pub mod subscribe;

use crate::{Error, Mode};
use ashpd::desktop::settings::ColorScheme as PortalColorScheme;
use ashpd::desktop::settings::Settings as XdgPortalSettings;

impl From<PortalColorScheme> for Mode {
    fn from(value: PortalColorScheme) -> Self {
        match value {
            PortalColorScheme::NoPreference => Mode::Unspecified,
            PortalColorScheme::PreferDark => Mode::Dark,
            PortalColorScheme::PreferLight => Mode::Light,
        }
    }
}

pub(crate) async fn get_color_scheme() -> Result<Mode, Error> {
    let settings = XdgPortalSettings::new().await?;
    let color_scheme = settings.color_scheme().await?;
    Ok(color_scheme.into())
}
