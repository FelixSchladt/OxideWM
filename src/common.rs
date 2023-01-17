use log::LevelFilter;

pub const LOG_LEVEL_ENV: &str = "RUST_LOG";
pub const LOG_LEVEL_DEFAULT: LevelFilter = LevelFilter::Info;

type ExitCode = i32;

pub const EXIT_CODE_LOGGER_CONFIG_FAIL: ExitCode = 1;
