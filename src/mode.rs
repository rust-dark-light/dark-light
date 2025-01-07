/// Enum representing dark mode, light mode, or unspecified.
///
/// If `Mode::Default` is returned, it is expected that the user decides which theme mode to use for their specific use case.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Mode {
    /// Represents the dark mode option.
    Dark,
    /// Represents the light mode option.
    Light,
    /// Used when the system theme mode is unspecified.
    Unspecified,
}

impl Mode {
    /// Returns `true` if the mode is `Mode::Dark`.
    pub fn is_dark(&self) -> bool {
        matches!(self, Mode::Dark)
    }

    /// Returns `true` if the mode is `Mode::Light`.
    pub fn is_light(&self) -> bool {
        matches!(self, Mode::Light)
    }

    /// Returns `true` if the mode is `Mode::Unspecified`.
    pub fn is_unspecified(&self) -> bool {
        matches!(self, Mode::Unspecified)
    }
}

impl From<bool> for Mode {
    fn from(dark: bool) -> Self {
        if dark {
            Self::Dark
        } else {
            Self::Light
        }
    }
}
