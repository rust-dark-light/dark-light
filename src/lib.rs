//! This crate is designed to facilitate the development of applications that support both light and dark themes. It provides a simple API to detect the current theme mode.
//!
//! It supports macOS, Windows, Linux, BSDs, and WASM.
//!
//! On Linux the [XDG Desktop Portal](https://flatpak.github.io/xdg-desktop-portal/) D-Bus API is checked for the `color-scheme` preference, which works in Flatpak sandboxes without needing filesystem access.

mod error;
mod mode;
mod platforms;
#[cfg(any(feature = "tokio", feature = "async-io"))]
mod stream;
mod watch;

pub use error::Error;
pub use mode::Mode;
pub use watch::Watcher;

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

/// Subscribes to changes in the system theme mode.
///
/// Returns a [`Watcher`] that receives a new [`Mode`] each time the OS theme changes.
/// Only mode transitions are emitted, and the watcher's background thread (where used)
/// stops when the `Watcher` is dropped.
///
/// # Example
///
/// ``` no_run
/// use dark_light::{ Error, Mode };
///
/// fn main() -> Result<(), Error> {
///     let watcher = dark_light::subscribe()?;
///     for mode in watcher.iter() {
///         match mode {
///             Mode::Dark => {},
///             Mode::Light => {},
///             Mode::Unspecified => {},
///         }
///     }
///     Ok(())
/// }
/// ```
pub use platforms::platform::subscribe;

/// Subscribes to changes in the system theme mode using an async stream.
///
/// Returns a [`Stream`] that yields a new [`Mode`] each time the OS theme changes.
///
/// # Example
///
/// ``` no_run
/// use dark_light::{ Error, Mode };
/// use futures::StreamExt;
///
/// async fn main() -> Result<(), Error> {
///     let mut stream = dark_light::stream()?;
///     while let Some(mode) = stream.next().await {
///         match mode {
///             Mode::Dark => {},
///             Mode::Light => {},
///             Mode::Unspecified => {},
///         }
///     }
///     Ok(())
/// }
/// ```
#[cfg(any(feature = "tokio", feature = "async-io"))]
pub use stream::stream;
