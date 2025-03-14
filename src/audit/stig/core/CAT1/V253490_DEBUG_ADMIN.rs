use windows_registry::*;
use windows_result::Error;

/*
use crate::audit::stig::core::framework::{NoLMHash,NoLMHashStatus};

pub struct V253461_SAM;

impl V253461_SAM {
    pub fn status() -> std::result::Result<NoLMHashStatus,Error> {
        let path = "SECURITY\\Policy\\Secrets";
        let key_name = "SeDebugPrivilege";

        let hklm = Key::open(CURRENT_USER, path);

        let value = match hklm {
            Ok(hklm) => hklm.get_u32(key_name),
            Err(_) => panic!("No Key Opened")
        };

        if value.is_ok() {
            match value.unwrap() {
                0u32 => Ok(NoLMHashStatus(NoLMHash::Disabled)),
                1u32 => Ok(NoLMHashStatus(NoLMHash::Enabled)),
                _ => Ok(NoLMHashStatus(NoLMHash::Corrupted)),
            }
        }
        else {
            return Err(value.unwrap_err())
        }

    }
}
    

#[test]
fn run_sam() {
    let status = V253461_SAM::status().unwrap();

    println!("Status: {:?}", status.0)
}

*/