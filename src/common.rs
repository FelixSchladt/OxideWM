use log::LevelFilter;

pub const PROJECT_NAME: &str = "oxidewm";

pub const LOG_LEVEL_ENV: &str = "RUST_LOG";
pub const LOG_LEVEL_DEFAULT: LevelFilter = LevelFilter::Info;
pub const LOG_FILE_NAME: &str = "oxidewm";
pub const LOG_FILE_EXTENSION: &str = "log";
pub const LOG_FILE_LOCATION_DEV: &str = "log/";

type ExitCode = i32;

pub const EXIT_CODE_LOGGER_CONFIG_FAIL: ExitCode = 1;
