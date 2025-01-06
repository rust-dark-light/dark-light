//! This crate is designed to facilitate the development of applications that support both light and dark themes. It provides a simple API to detect the current theme mode and subscribe to changes in the system theme mode.
//!
//! It supports macOS, Windows, Linux, BSDs, and WASM.
//!
//! On Linux the [XDG Desktop Portal](https://flatpak.github.io/xdg-desktop-portal/) D-Bus API is checked for the `color-scheme` preference, which works in Flatpak sandboxes without needing filesystem access.
mod mode;
mod platforms;

pub use mode::Mode;

/// Detects the system theme mode. If the mode can’t be detected, it fallbacks to [`Mode::Default`].
///
/// # Example
///
/// ``` no_run
/// use dark_light::Mode;
///
/// let mode = dark_light::detect();
///
/// match mode {
///     Mode::Dark => {},
///     Mode::Light => {},
///     Mode::Default => {},
/// }
/// ```
pub use platforms::platform::detect::detect;

/// Notifies the user if the system theme has been changed.
///
/// This function returns a stream of `Mode` values. The stream will emit a new value whenever the system theme changes.
///
/// # Example
///
/// ``` no_run
/// use dark_light::Mode;
/// use futures_lite::stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let mut stream = dark_light::subscribe().await;
///
///     while let Some(mode) = stream.next().await {
///         match mode {
///             Mode::Dark => {},
///             Mode::Light => {},
///             Mode::Default => {},
///         }
///     }
/// }
/// ```
pub use platforms::platform::subscribe::subscribe;
