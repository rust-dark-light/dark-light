// Dark/light mode detection on macOS.
// Written with help from Ryan McGrath (https://rymc.io/).

#[cfg(any(feature = "sync", doc))]
pub mod sync {
    pub fn detect() -> crate::Mode {
        super::super::is_dark_mode().into()
    }
}

pub async fn detect() -> crate::Mode {
    super::is_dark_mode().into()
}
