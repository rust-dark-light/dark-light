// Dark/light mode detection on macOS.
// Written with help from Ryan McGrath (https://rymc.io/).

pub fn detect() -> crate::Mode {
    crate::Mode::from_bool(super::is_dark_mode())
}
