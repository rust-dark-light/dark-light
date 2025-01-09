//! This crate is designed to facilitate the development of applications that support both light and dark themes. It provides a simple API to detect the current theme mode.
//!
//! It supports macOS, Windows, Linux, BSDs, and WASM.
//!
//! On Linux the [XDG Desktop Portal](https://flatpak.github.io/xdg-desktop-portal/) D-Bus API is checked for the `color-scheme` preference, which works in Flatpak sandboxes without needing filesystem access.

mod error;
mod mode;
mod platforms;

pub use error::Error;
pub use mode::Mode;

/// Detects the system theme mode.
///
/// # Example
///
/// ``` no_run
/// use dark_light::{ Error, Mode };
///
/// fn main() -> Result<(), Error> {
///     let mode = dark_light::detect()?;
///     match mode {
///         Mode::Dark => {},
///         Mode::Light => {},
///         Mode::Unspecified => {},
///     }
///     Ok(())
/// }
/// ```
pub use platforms::platform::detect;
