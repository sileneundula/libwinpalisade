use windows_registry::*;

use crate::audit::stig::core::framework::{Autoplay,AutoplayTypes};
use windows_result::Error;

pub struct V253386API;

impl V253386API {
    pub fn status() -> Autoplay {
        let path = "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\AutoplayHandlers";
        let key_name = "DisableAutoplay";
        let hklm = Key::open(CURRENT_USER, path);

        let value = match hklm {
            Ok(hklm) => hklm.get_u32(key_name),
            Err(_) => panic!("No Key Opened")
        };

        let reg_value = match value {
            Ok(value) => value,
            Err(_) => panic!("Could Not Read Value")
        };

        if reg_value == 1u32 {
            println!("Autoplay Disabled");
            return Autoplay(AutoplayTypes::AutoplayDisabled)
        }
        else if reg_value == 0u32 {
            println!("Autoplay not Disabled");
            return Autoplay(AutoplayTypes::AutoplayEnabled)
        }
        else {
            return Autoplay(AutoplayTypes::Corrupted)
        }
    }
    pub fn enable() -> std::result::Result<bool,Error> {
        let path = "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\AutoplayHandlers";
        let key_name = "DisableAutoplay";
        let hklm = Key::open(CURRENT_USER, path);

        let value = match hklm {
            Ok(hklm) => hklm.set_u32(key_name, 1u32),
            Err(_) => panic!("No Key Opened")
        };

        if value.is_ok() {
            return Ok(true)
        }
        else {
            return Err(value.unwrap_err())
        }
    }
    pub fn disable() -> std::result::Result<bool,Error> {
        let path = "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\AutoplayHandlers";
        let key_name = "DisableAutoplay";
        let hklm = Key::open(CURRENT_USER, path);

        let value = match hklm {
            Ok(hklm) => hklm.set_u32(key_name, 0u32),
            Err(_) => panic!("No Key Opened")
        };

        if value.is_ok() {
            return Ok(true)
        }
        else {
            return Err(value.unwrap_err())
        }
    }

}

#[test]
fn run() {
    V253386API::status();
}