/// Enum representing dark mode, light mode, or unspecified.
///
/// If `Mode::Unspecified` is returned, it is expected that the user decides which theme mode to use for their specific use case.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Mode {
    /// Represents the dark mode option.
    Dark,
    /// Represents the light mode option.
    Light,
    /// Used when the system theme mode is unspecified.
    Unspecified,
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
