extern crate log;

use syslog::{Facility, Formatter3164, BasicLogger};
use log::{LevelFilter, info};


fn test() {
    let formatter = Formatter3164 {
      facility: Facility::LOG_USER,
      hostname: None,
      process: "myprogram".into(),
      pid: 42,
    };
  
    match syslog::unix(formatter) {
      Err(e)         => println!("impossible to connect to syslog: {:?}", e),
      Ok(mut writer) => {
        writer.err("hello world").expect("could not write error message");
      }
    }
  }


fn main (){
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "oxide".into(),
        pid: 0,
    };

    let logger = match syslog::unix(formatter) {
        Err(e) => { println!("impossible to connect to syslog: {:?}", e); return; },
        Ok(logger) => logger,
    };
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
            .map(|()| log::set_max_level(LevelFilter::Info)).expect("failed to set boxed logger");

    info!("hello world");
    println!("something was logged")
}