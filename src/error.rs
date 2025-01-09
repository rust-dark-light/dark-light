use std::fmt::Display;

/// An error that can occur when detecting the system theme mode.
#[derive(Debug)]
pub enum Error {
    /// If an I/O error occurs.
    Io(std::io::Error),
    /// If the system theme mode could not be detected.
    DetectionFailed,
    /// If the XDG Desktop Portal could not be communicated with.
    XdgDesktopPortal(String),
    /// If the timeout is reached.
    Timeout,
    /// Failed to get persistent domain for Apple Global Domain
    PersistentDomainFailed,
    /// Failed to get AppleInterfaceStyle
    AppleInterfaceStyleFailed,
    /// If the window could not be found.
    WindowNotFound,
    /// If the media query could not be executed.
    MediaQueryFailed,
    /// If the media query is not supported.
    MediaQueryNotSupported,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(error) => write!(f, "I/O error: {}", error),
            Error::DetectionFailed => write!(f, "Failed to detect system theme mode"),
            Error::XdgDesktopPortal(err) => write!(f, "XDG Desktop Portal error: {}", err),
            Error::Timeout => write!(f, "Timeout reached"),
            Error::PersistentDomainFailed => {
                write!(f, "Failed to get persistent domain for Apple Global Domain")
            }
            Error::AppleInterfaceStyleFailed => write!(f, "Failed to get AppleInterfaceStyle"),
            Error::WindowNotFound => write!(f, "Window not found"),
            Error::MediaQueryFailed => write!(f, "Media query failed"),
            Error::MediaQueryNotSupported => write!(f, "Media query not supported"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}
