// Dark/light mode detection on macOS.
// Written with help from Ryan McGrath (https://rymc.io/).

use crate::{Error, Mode};
use objc2_foundation::{ns_string, NSString, NSUserDefaults};

pub fn detect() -> Result<Mode, Error> {
    let style = NSUserDefaults::standardUserDefaults()
        .persistentDomainForName(ns_string!("Apple Global Domain"))
        .ok_or(Error::PersistentDomainFailed)?
        .objectForKey(ns_string!("AppleInterfaceStyle"));

    let Some(style) = style else {
        return Ok(Mode::Light);
    };

    let Ok(style) = style.downcast::<NSString>() else {
        return Err(Error::PersistentDomainFailed);
    };
    let mode = style.isEqualToString(ns_string!("Dark")).into();
    Ok(mode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_returns_mode() {
        let result = detect();
        assert!(result.is_ok(), "detect() should return a valid mode");
        let mode = result.unwrap();
        if !matches!(mode, Mode::Light | Mode::Dark) {
            eprintln!("Warning: Unexpected mode value: {:?}", mode);
        }
    }
}
