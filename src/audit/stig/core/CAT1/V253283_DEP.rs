use windows::Win32::System::SystemInformation::{GetSystemDEPPolicy, DEP_SYSTEM_POLICY_TYPE};

use crate::audit::stig::core::framework::{DEPStatusTypes,DEPStatusV253283};

///! DEP
/// 
/// 0: DEP is set to off (Security Warning)
/// 1: DEP is set to on (Best)
/// 2: DEP is set to Opt-In
/// 3: DEP is set to Opt-Out

/// DEP Must Be Enabled (At least to Optout)
pub struct V253283API;

impl V253283API {
    pub fn status() -> DEPStatusV253283 {
        
        unsafe {
            let policy: DEP_SYSTEM_POLICY_TYPE = GetSystemDEPPolicy();

                match policy.0 {
                    0i32 => println!("DEP is set to Always Off"),
                    1i32 => println!("DEP is set to Always On"),
                    2i32 => println!("DEP is set to Opt-In"),
                    3i32 => println!("DEP is set to Opt-Out"),
                    _ => panic!("Error with integer during DEP")
                }

                let dep_status = match policy.0.clone() {
                    0i32 => DEPStatusV253283(DEPStatusTypes::DataExecutionPreventionAlwaysOff),
                    1i32 => DEPStatusV253283(DEPStatusTypes::DateExecutionPreventionAlwaysOn),
                    2i32 => DEPStatusV253283(DEPStatusTypes::DataExecutionPreventionOptIn),
                    3i32 => DEPStatusV253283(DEPStatusTypes::DataExecutionPreventionOptOut),
                    _ => panic!("Failure")
                };

                return dep_status
        }
    }
}

#[test]
fn dep() {
    V253283API::status();
}