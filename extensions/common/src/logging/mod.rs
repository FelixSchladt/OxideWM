pub mod teelogger;

use std::{process, str::FromStr};

use log::{info, LevelFilter};

#[cfg(debug_assertions)]
use log4rs::{
    append::{
        console::ConsoleAppender,
        rolling_file::{
            policy::compound::{
                roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
            },
            RollingFileAppender,
        },
    },
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

use syslog::{Facility, Formatter3164};

use self::teelogger::Teelogger;

const _LOG_FILE_EXTENSION: &str = "log";
const LOG_LEVEL_ENV: &str = "OXIDE_LOG";

pub fn init_logger(
    log_level: LevelFilter,
    _log_file_name: String,
    _log_file_location: String,
    process_name: String,
) {
    let mut tee_logger = Teelogger::new();

    #[cfg(debug_assertions)]
    {
        let log4rs_logger = get_log4rs_logger(log_level, _log_file_name, _log_file_location);
        tee_logger.add_logger(log4rs_logger);
    }

    let sys_logger = get_sys_logger(process_name);
    tee_logger.add_logger(sys_logger);

    let result =
        log::set_boxed_logger(Box::new(tee_logger)).map(|()| log::set_max_level(log_level));

    if result.is_err() {
        println!("failed to set boxed logger");
    } else {
        info!("Logging with Loglevel {}", log_level);
    };
}

pub fn get_log_level() -> Result<LevelFilter, ()> {
    let log_level_env = std::env::var(LOG_LEVEL_ENV.to_string());
    if let Ok(level_env) = log_level_env {
        let log_level = LevelFilter::from_str(level_env.as_str());
        if let Ok(level) = log_level {
            Ok(level)
        } else {
            eprintln!("Could not parse log level from {}", level_env);
            Err(())
        }
    } else {
        Err(())
    }
}

#[cfg(debug_assertions)]
fn get_log_file_appender(log_file_name: String, log_file_location: String) -> RollingFileAppender {
    use std::{fs, path::PathBuf};

    let log_file_pattern = format!(
        "{}{}{{}}.{}",
        log_file_location, log_file_name, _LOG_FILE_EXTENSION
    );

    let log_file = format!(
        "{}{}.{}",
        log_file_location, log_file_name, _LOG_FILE_EXTENSION
    );

    fs::create_dir_all(PathBuf::from(log_file.clone()).parent().unwrap())
        .expect("failed to create dir for logging");

    let window_size = 3; // log0, log1, log2
    let fixed_window_roller = FixedWindowRoller::builder()
        .build(log_file_pattern.as_str(), window_size)
        .unwrap();

    let size_limit = 5 * u64::pow(2, 20); // 5MB as max log file size to roll
    let size_trigger = SizeTrigger::new(size_limit);

    let compound_policy =
        CompoundPolicy::new(Box::new(size_trigger), Box::new(fixed_window_roller));

    RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)(utc)} - {l}: {m}{n}",
        )))
        .build(log_file, Box::new(compound_policy))
        .unwrap()
}

#[cfg(debug_assertions)]
fn get_log4rs_logger(
    log_level: LevelFilter,
    log_file_name: String,
    log_file_location: String,
) -> Box<dyn log::Log> {
    let stdout_appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)(utc)} - [{h({l})}]: {m}{n}",
        )))
        .build();

    let log_file_appender = get_log_file_appender(log_file_name, log_file_location);

    let config = log4rs::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout_appender)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(log_level)))
                .build("logfile", Box::new(log_file_appender)),
        )
        .build(
            Root::builder()
                .appender("stdout")
                .appender("logfile")
                .build(log_level),
        )
        .unwrap();
    Box::new(log4rs::Logger::new(config))
}

fn get_sys_logger(process_name: String) -> Box<dyn log::Log> {
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: process_name,
        pid: process::id(),
    };

    let logger = syslog::unix(formatter).expect("could not connect to syslog");
    Box::new(syslog::BasicLogger::new(logger))
}
