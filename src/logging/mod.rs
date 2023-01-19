pub mod teelogger;

use std::{process, str::FromStr};

use log::{LevelFilter, info};

#[cfg(debug_assertions)]
use log4rs::{
    append::{
        console::ConsoleAppender,
        rolling_file::{
            RollingFileAppender,
            policy::compound::{
                CompoundPolicy,
                roll::fixed_window::FixedWindowRoller,
                trigger::size::SizeTrigger
            }
        }
    },
    filter::threshold::ThresholdFilter,
    encode::pattern::PatternEncoder,
    config::{Appender, Root},
};
use syslog::{Formatter3164, Facility};

use crate::common;

use self::teelogger::Teelogger;

pub fn init_logger(){
    let log_level = get_log_level();
    let mut tee_logger = Teelogger::new();

    #[cfg(debug_assertions)]
    {
        let log4rs_logger = get_log4rs_logger(log_level);
        tee_logger.add_logger(log4rs_logger);
    }

    let sys_logger = get_sys_logger();
    tee_logger.add_logger(sys_logger);

    let result = log::set_boxed_logger(Box::new(tee_logger))
        .map(|()| log::set_max_level(log_level));

    if result.is_err() {
        println!("failed to set boxed logger");
    }else{
        info!("Logging with Loglevel {}", log_level);
    };
}

fn get_log_level() -> LevelFilter {
    let log_level_env = std::env::var(common::LOG_LEVEL_ENV.to_string());
    if let Ok(level_env) = log_level_env{
        let log_level = LevelFilter::from_str(level_env.as_str());
        if let Ok(level) = log_level {
            level
        }else{
            eprintln!("Could not parse log level from {}", level_env);
            common::LOG_LEVEL_DEFAULT
        }
    }else{
        common::LOG_LEVEL_DEFAULT
    }
}

#[cfg(debug_assertions)]
fn get_log_file_appender()->RollingFileAppender{
    let log_path = common::LOG_FILE_LOCATION_DEV;

    let log_file_pattern = format!("{}{}{{}}.{}",log_path,common::LOG_FILE_NAME,common::LOG_FILE_EXTENSION);
    let log_file = format!("{}{}.{}",log_path,common::LOG_FILE_NAME,common::LOG_FILE_EXTENSION);

    let window_size = 3; // log0, log1, log2
    let fixed_window_roller = FixedWindowRoller::builder().build(log_file_pattern.as_str(),window_size).unwrap();

    let size_limit = 5 * u64::pow(2, 20); // 5MB as max log file size to roll
    let size_trigger = SizeTrigger::new(size_limit);

    let compound_policy = CompoundPolicy::new(Box::new(size_trigger),Box::new(fixed_window_roller));

    RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)(utc)} - {l}: {m}{n}")))
        .build(log_file, Box::new(compound_policy))
        .unwrap()
}

#[cfg(debug_assertions)]
fn get_log4rs_logger(log_level:LevelFilter) -> Box<dyn log::Log> {
    let stdout_appender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)(utc)} - [{h({l})}]: {m}{n}")))
        .build();

    let config = log4rs::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout_appender)))
        .appender(Appender::builder()
            .filter(Box::new(ThresholdFilter::new(log_level)))
            .build("logfile", Box::new(get_log_file_appender()))
        )
        .build(
            Root::builder()
            .appender("stdout")
            .appender("logfile")
            .build(log_level)
        )
        .unwrap();
    Box::new(log4rs::Logger::new(config))
}

fn get_sys_logger() -> Box<dyn log::Log> {
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: common::PROJECT_NAME.into(),
        pid: process::id(),
    };

    let logger = syslog::unix(formatter).expect("could not connect to syslog");
    Box::new(syslog::BasicLogger::new(logger))
}