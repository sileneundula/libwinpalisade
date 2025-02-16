use pretty_env_logger;
#[macro_use] use log::*;

// #1. DEP
#[derive(Clone,Copy)]
pub struct DEPStatusV253283(pub DEPStatusTypes);

#[derive(Clone,Copy)]
pub enum DEPStatusTypes {
    DateExecutionPreventionAlwaysOn,
    DataExecutionPreventionAlwaysOff,
    DataExecutionPreventionOptIn,
    DataExecutionPreventionOptOut,
}

impl DEPStatusV253283 {
    pub fn status(&self) -> DEPStatusTypes {
        return self.0
    }
    pub fn audit(&self) {
        match self.0 {
            DEPStatusTypes::DateExecutionPreventionAlwaysOn => info!("[CAT I: V-253283] DEP set to Always On. Highest Security Level Achieved For DEP."),
            DEPStatusTypes::DataExecutionPreventionAlwaysOff => warn!("[High Severity Warning] [CAT I: V-253283] DEP set to Always Off"),
            DEPStatusTypes::DataExecutionPreventionOptIn => warn!("[High Severity Warning] [CAT I: V-253283] DEP set to Opt-In. Only certain binaries are secure."),
            DEPStatusTypes::DataExecutionPreventionOptOut => info!("[CAT I: V-253283] DEP set to Opt-Out")
        }
    }
}