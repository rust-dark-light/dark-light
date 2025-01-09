use crate::{Error, Mode};
use winreg::RegKey;

const SUBKEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize";
const VALUE: &str = "AppsUseLightTheme";

pub fn detect() -> Result<Mode, Error> {
    let hkcu = RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let subkey = hkcu.open_subkey(SUBKEY)?;
    let dword: u32 = subkey.get_value(VALUE)?;
    Ok((dword == 0).into())
}
