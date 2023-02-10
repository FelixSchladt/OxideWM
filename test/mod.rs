pub fn in_pipeline() -> bool {
    match std::env::var("GITHUB_PIPELINE") {
        Ok(_)  => true,
        Err(_) => false,
    }
}

pub mod config_tests;
pub mod screeninfo_tests;
