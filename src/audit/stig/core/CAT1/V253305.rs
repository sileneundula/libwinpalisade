use windows_registry::*;

pub struct V253284API;

//TODO: Add Checkup on if registry key exists for value

impl V253284API {
    pub fn status() -> Result<bool> {
        let key_path = "SYSTEM\\CurrentControlSet\\Control\\Lsa";
        let value_name = "ClearTextPassword";
        let hklm = Key::open(LOCAL_MACHINE, key_path);

        let value = match hklm {
            Ok(hklm) => hklm.get_u32(value_name),
            Err(_) => panic!("Cannot Open Critical Registry Key Or Not Found"),
        };

        match value {
            Ok(value) => Ok(value != 0u32),
            Err(_) => Ok(false)
        }
    }
}

#[test]
fn get_status() {
    let status = V253284API::status().unwrap();

    println!("Stauts: {}",status);
}