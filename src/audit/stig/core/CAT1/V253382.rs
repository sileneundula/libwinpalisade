use windows_registry::*;
use log::*;


use crate::audit::stig::core::framework::{RemoteDesktopAssistanceStatus,RemoteDesktopAssistanceStatusTypes};

pub struct V253382API;

impl V253382API {
    pub fn status() -> RemoteDesktopAssistanceStatus {
        let path = "SYSTEM\\CurrentControlSet\\Control\\Remote Assistance";
        let key_name = "fAllowToGetHelp";

        let hklm = Key::open(LOCAL_MACHINE, path);

        let value = match hklm {
            Ok(hklm) => hklm.get_u32(key_name),
            Err(_) => panic!("Failed To Open Critical RegKey"),
        };

        let reg_value = match value {
            Ok(value) => value,
            Err(_) => panic!("Failed To Get Remote Desktop Assistance")
        };

        if reg_value == 0u32 {
            println!("Remote Desktop Assistance is Disabled");
            return RemoteDesktopAssistanceStatus(RemoteDesktopAssistanceStatusTypes::Disabled)
        }
        else if reg_value == 1u32 {
            println!("Remote Desktop Assistance is Enabled");

            return RemoteDesktopAssistanceStatus(RemoteDesktopAssistanceStatusTypes::Enabled)
        }
        else {
            println!("Remote Desktop Assistance is ");
            return RemoteDesktopAssistanceStatus(RemoteDesktopAssistanceStatusTypes::Corrupted)
        }
    }
}

#[test]
fn run() {
    V253382API::status();
}