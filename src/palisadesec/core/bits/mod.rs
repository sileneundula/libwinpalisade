use windows::{
    core::*,
    Win32::System::Com::*,
    Win32::Networking::BackgroundIntelligentTransferService::*
};

fn bits_jobs() -> Result<()> {
    // Initialize COM library
    unsafe {
        // Initialize COM
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;
        
        // Manager
        let manager: IBackgroundCopyManager =
        CoCreateInstance(&BackgroundCopyManager, None, CLSCTX_LOCAL_SERVER)?;

        let jobs = manager.EnumJobs(BG_JOB_ENUM_ALL)?;

        let job_count = jobs.GetCount().unwrap();

        println!("Job Count: {}", job_count);

        for i in 0..job_count {
            let mut job = None;
            jobs.Next(&mut job, 1)?;

            if let Some(job) = job {
                let mut display_name = windows::core::PWSTR::null();
                let display_name = job.GetDisplayName()?;

                // Convert to a Rust string
                let display_name_str = display_name.to_string().unwrap_or_else("Unknown".to_string());
                println!("Job {}: {}", i + 1, display_name_str);
            }
        }
        CoUninitialize();
    }

    Ok(())
}

#[test]
fn create_bits_jobs() {
    let x = bits_jobs();
}