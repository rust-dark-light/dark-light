// Dark/light mode detection on macOS.
// Written with help from Ryan McGrath (https://rymc.io/).

use objc2::{class, msg_send};
use objc2_foundation::{NSObject, NSString};

pub fn detect() -> Result<crate::Mode, crate::Error> {
    unsafe {
        let user_defaults: *mut NSObject = msg_send![class!(NSUserDefaults), standardUserDefaults];
        let apple_domain = NSString::from_str("Apple Global Domain");
        let dict: *mut NSObject = msg_send![user_defaults, persistentDomainForName:&*apple_domain];

        if dict.is_null() {
            return Err(crate::Error::PersistentDomainFailed);
        }

        let style_key = NSString::from_str("AppleInterfaceStyle");
        let style: *mut NSObject = msg_send![dict, objectForKey:&*style_key];

        if style.is_null() {
            return Err(crate::Error::AppleInterfaceStyleFailed);
        }

        // Compare with "Dark"
        let dark_str = NSString::from_str("Dark");
        let is_dark: bool = msg_send![style, isEqualToString:&*dark_str];
        Ok(is_dark.into())
    }
}
