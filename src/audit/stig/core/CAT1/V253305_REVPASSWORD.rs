use windows_registry::*;
use windows_result::Error;

// TODO: Requires Group Policy

pub struct V253284API;

//TODO: Add Checkup on if registry key exists for value

impl V253284API {
    pub fn status() -> std::result::Result<bool,Error> {
        let key_path = "SYSTEM\\CurrentControlSet\\Control\\Lsa";
        let value_name = "ClearTextPassword";
        let hklm = Key::open(LOCAL_MACHINE, key_path);

        let value = match hklm {
            Ok(hklm) => hklm.get_u32(value_name),
            Err(hklm) => return Err(hklm)
        };

        let reg_value = match value {
            Ok(value) => value,
            Err(value) => return Err(value)
        };

        if reg_value == 0u32 {
            return Ok(true)
        }
        else if reg_value == 1u32 {
            return Ok(false)
        }
        else {
            return Ok(false)
        }


    }
}

#[test]
fn get_status() {
    let status = V253284API::status().unwrap();

    println!("Stauts: {}",status);
}