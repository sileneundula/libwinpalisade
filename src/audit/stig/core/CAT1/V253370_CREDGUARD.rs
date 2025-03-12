
use windows_registry::*;
use windows_result::Error;

use crate::audit::stig::core::framework::{SEHOPStatus,SEHOPStatusTypes};

pub struct V253284API;

impl V253284API {
    pub fn status() -> std::result::Result<SEHOPStatus,Error> {
        let key_path = "SOFTWARE\\Policies\\Microsoft\\Windows\\DeviceGuard";
        let value_name = "LsaCfgFlags";
        let hklm: std::result::Result<Key, Error> = Key::open(LOCAL_MACHINE, key_path);

        let value: std::result::Result<u32, Error> = match hklm {
            Ok(hklm) => hklm.get_u32(value_name),
            Err(hklm) => Err(hklm)
        };

        let reg_value = match value {
            Ok(value) => value,
            Err(value) => return Err(value)
        };

        if reg_value == 0u32 {
            println!("Not Approved");
            return Ok(SEHOPStatus(SEHOPStatusTypes::Enabled))
        }
        else if reg_value == 1u32 {
            println!("Credential Guard Registry Entry (UEFI Lock)");
            return Ok(SEHOPStatus(SEHOPStatusTypes::Disabled))
        }
        else {
            println!("SEHOP Corrupted");
            return Ok(SEHOPStatus(SEHOPStatusTypes::Corrupted))
        }
    }
}

#[test]
fn run() {
    V253284API::status();
}