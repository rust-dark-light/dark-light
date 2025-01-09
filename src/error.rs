use std::fmt::Display;

/// An error that can occur when detecting the system theme mode.
#[derive(Debug)]
pub enum Error {
    /// If an I/O error occurs.
    Io(std::io::Error),
    /// If the system theme mode could not be detected.
    DetectionFailed,

    #[cfg(any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "dragonfly",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    /// If the XDG Desktop Portal could not be communicated with.
    XdgDesktopPortal(ashpd::Error),

    #[cfg(any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "dragonfly",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    /// If the timeout is reached.
    Timeout,

    /// Failed to get persistent domain for Apple Global Domain
    #[cfg(target_os = "macos")]
    PersistentDomainFailed,
    /// Failed to get AppleInterfaceStyle
    #[cfg(target_os = "macos")]
    AppleInterfaceStyleFailed,

    #[cfg(target_arch = "wasm32")]
    /// If the window could not be found.
    WindowNotFound,
    #[cfg(target_arch = "wasm32")]
    /// If the media query could not be executed.
    MediaQueryFailed,
    #[cfg(target_arch = "wasm32")]
    /// If the media query is not supported.
    MediaQueryNotSupported,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(error) => write!(f, "I/O error: {}", error),
            Error::DetectionFailed => write!(f, "Failed to detect system theme mode"),
            #[cfg(any(
                target_os = "linux",
                target_os = "freebsd",
                target_os = "dragonfly",
                target_os = "netbsd",
                target_os = "openbsd"
            ))]
            Error::XdgDesktopPortal(err) => write!(f, "XDG Desktop Portal error: {}", err),
            #[cfg(any(
                target_os = "linux",
                target_os = "freebsd",
                target_os = "dragonfly",
                target_os = "netbsd",
                target_os = "openbsd"
            ))]
            Error::Timeout => write!(f, "Timeout reached"),
            #[cfg(target_os = "macos")]
            Error::PersistentDomainFailed => {
                write!(f, "Failed to get persistent domain for Apple Global Domain")
            }
            #[cfg(target_os = "macos")]
            Error::AppleInterfaceStyleFailed => write!(f, "Failed to get AppleInterfaceStyle"),
            #[cfg(target_arch = "wasm32")]
            Error::WindowNotFound => write!(f, "Window not found"),
            #[cfg(target_arch = "wasm32")]
            Error::MediaQueryFailed => write!(f, "Media query failed"),
            #[cfg(target_arch = "wasm32")]
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

#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "netbsd",
    target_os = "openbsd"
))]
impl From<ashpd::Error> for Error {
    fn from(error: ashpd::Error) -> Self {
        Error::XdgDesktopPortal(error)
    }
}
