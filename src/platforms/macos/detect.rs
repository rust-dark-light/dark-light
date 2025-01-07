// Dark/light mode detection on macOS.
// Written with help from Ryan McGrath (https://rymc.io/).

pub async fn detect() -> Result<crate::Mode, crate::Error> {
    Ok(super::is_dark_mode().into())
}
