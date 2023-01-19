use log::{Log, Record, Metadata};

pub struct Teelogger{
    loggers:Vec<Box<dyn Log>>
}

impl Teelogger {
    pub fn new()-> Teelogger{
        Teelogger { loggers: vec![] }
    }

    pub fn add_logger(&mut self, logger: Box<dyn Log>)->&mut Self{
        self.loggers.push(logger);
        self
    }
}

#[allow(unused_variables, unused_must_use)]
impl Log for Teelogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        for logger in &self.loggers {
            if logger.enabled(metadata) {
                return true;
            }
        }
        false
    }

    fn log(&self, record: &Record) {
        for logger in &self.loggers{
            if logger.enabled(record.metadata()){
                logger.log(record);
            }
        }
    }

    fn flush(&self) {
        for logger in &self.loggers{
            let _ = logger.flush();
        }
    }
}