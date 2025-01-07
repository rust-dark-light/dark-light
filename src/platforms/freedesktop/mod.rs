pub mod detect;
pub mod subscribe;

use ashpd::desktop::settings::ColorScheme as PortalColorScheme;
use ashpd::desktop::settings::Settings as XdgPortalSettings;

impl From<PortalColorScheme> for crate::Mode {
    fn from(value: PortalColorScheme) -> Self {
        match value {
            PortalColorScheme::NoPreference => crate::Mode::Unspecified,
            PortalColorScheme::PreferDark => crate::Mode::Dark,
            PortalColorScheme::PreferLight => crate::Mode::Light,
        }
    }
}

pub(crate) async fn get_color_scheme() -> Result<crate::Mode, crate::Error> {
    let settings = XdgPortalSettings::new().await?;
    let color_scheme = settings.color_scheme().await?;
    Ok(color_scheme.into())
}
