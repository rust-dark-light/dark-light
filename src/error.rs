use thiserror::Error;

/// An error that can occur when detecting the system theme mode.
#[derive(Debug, Error)]
pub enum Error {
    /// If an I/O error occurs.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    /// If the system theme mode could not be detected.
    #[error("The system theme mode could not be detected")]
    DetectionFailed,

    #[cfg(target_os = "linux")]
    /// If the XDG Desktop Portal could not be communicated with.
    #[error("Failed to communicate with the XDG Desktop Portal: {0}")]
    XdgDesktopPortal(#[from] ashpd::Error),

    /// Failed to get persistent domain for Apple Global Domain
    #[cfg(target_os = "macos")]
    #[error("Failed to get persistent domain for Apple Global Domain")]
    PersistentDomainFailed,
    /// Failed to get AppleInterfaceStyle
    #[cfg(target_os = "macos")]
    #[error("Failed to get AppleInterfaceStyle")]
    AppleInterfaceStyleFailed,

    #[cfg(target_arch = "wasm32")]
    /// If the window could not be found.
    #[error("The window could not be found")]
    WindowNotFound,
    #[cfg(target_arch = "wasm32")]
    /// If the media query could not be executed.
    #[error("The media query could not be executed")]
    MediaQueryFailed,
    #[cfg(target_arch = "wasm32")]
    /// If the media query is not supported.
    #[error("The media query is not supported")]
    MediaQueryNotSupported,
}
