#![deny(clippy::pedantic)]

pub mod eventhandler;
pub mod windowmanager;
pub mod workspace;
pub mod windowstate;
pub mod screeninfo;
pub mod config;
pub mod keybindings;
pub mod auxiliary;
pub mod ipc;
pub mod common;
pub mod teelogger;

use std::str::FromStr;
use std::sync::{Arc, Mutex};

use std::sync::mpsc::channel;
use std::{thread, process};
use std::{cell::RefCell, rc::Rc};

use config::Config;
use serde_json::Result;

use log::{LevelFilter, error, trace};
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
use teelogger::Teelogger;

use crate::{
    windowmanager::WindowManager,
    eventhandler::EventHandler,
    keybindings::KeyBindings,
    eventhandler::events::IpcEvent,
    ipc::zbus_serve,
};

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

fn get_log_file_appender()->RollingFileAppender{
    #[cfg(debug_assertions)]
    let log_path = common::LOG_FILE_LOCATION_DEV;
    #[cfg(not(debug_assertions))]
    let log_path = common::LOG_FILE_LOCATION_PROD;

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

fn init_logger(){
    let log_level = get_log_level();

    let log4rs_logger = get_log4rs_logger(log_level);
    let sys_logger = get_sys_logger();

    let mut tee_logger = Teelogger::new();

    tee_logger.add_logger(log4rs_logger)
        .add_logger(sys_logger);

    let result = log::set_boxed_logger(Box::new(tee_logger))
        .map(|()| log::set_max_level(LevelFilter::Info));

    if result.is_err() {
        println!("failed to set boxed logger");
    };
}

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
        process: "oxide-wm".into(),
        pid: process::id(),
    };

    let logger = syslog::unix(formatter).expect("could not connect to syslog");
    Box::new(syslog::BasicLogger::new(logger))
}

fn main() -> Result<()> {
    init_logger();

    let mut config = Rc::new(RefCell::new(Config::new()));
    let mut keybindings = KeyBindings::new(&config.borrow());
    
    let mut manager = WindowManager::new(&keybindings, config.clone());
    let mut eventhandler = EventHandler::new(&mut manager, &keybindings);

    let (ipc_sender, wm_receiver) = channel::<IpcEvent>();
    let (wm_sender, ipc_receiver) = channel::<String>();


    let ipc_sender_mutex = Arc::new(Mutex::new(ipc_sender));
    let ipc_receiver_mutex = Arc::new(Mutex::new(ipc_receiver));

    thread::spawn(move || {
        async_std::task::block_on(zbus_serve(ipc_sender_mutex, ipc_receiver_mutex)).unwrap();
    });

    loop {
        let result = eventhandler.window_manager.poll_for_event();
        if let Ok(Some(event)) = result {
            eventhandler.handle_event(&event);
        } else {
            if let Some(error) = result.err(){
                error!("Error retreiving Event from Window manager {:?}", error);
            }
        }

        if let Ok(event) = wm_receiver.try_recv() {
            if event.status {
                let wm_state = eventhandler.window_manager.get_state();
                let j = serde_json::to_string(&wm_state)?;
                trace!("IPC status request");
                wm_sender.send(j).unwrap();
            } else {
                eventhandler.handle_ipc_event(event);
            }
        }

        if eventhandler.window_manager.restart {
            config = Rc::new(RefCell::new(Config::new()));
            keybindings = KeyBindings::new(&config.borrow());

            eventhandler = EventHandler::new(&mut manager, &keybindings);
            eventhandler.window_manager.restart_wm(&keybindings, config.clone());
        }
    }
}
