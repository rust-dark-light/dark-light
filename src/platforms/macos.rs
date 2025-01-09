// Dark/light mode detection on macOS.
// Written with help from Ryan McGrath (https://rymc.io/).

use crate::{Error, Mode};
use objc2::rc::Retained;
use objc2_foundation::{ns_string, NSString, NSUserDefaults};

pub fn detect() -> Result<Mode, Error> {
    unsafe {
        let style = NSUserDefaults::standardUserDefaults()
            .persistentDomainForName(ns_string!("Apple Global Domain"))
            .ok_or(Error::PersistentDomainFailed)?
            .objectForKey(ns_string!("AppleInterfaceStyle"));

        let Some(style) = style else {
            return Ok(Mode::Light);
        };

        let style = Retained::cast::<NSString>(style);
        let mode = style.isEqualToString(ns_string!("Dark")).into();
        Ok(mode)
    }
}
