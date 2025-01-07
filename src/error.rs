use thiserror::Error;

/// An error that can occur when detecting the system theme mode.
#[derive(Debug, Error)]
pub enum Error {
    /// If the system theme mode could not be detected.
    #[error("The system theme mode could not be detected")]
    DetectionFailed,
    #[cfg(target_os = "linux")]
    /// If the XDG Desktop Portal could not be communicated with.
    #[error("Failed to communicate with the XDG Desktop Portal: {0}")]
    XdgDesktopPortal(#[from] ashpd::Error),
}
