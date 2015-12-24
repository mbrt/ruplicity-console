#![macro_use]

use std::io::Write;
use log;

macro_rules! fatal {
    ($($arg:tt)+) => {
        error!($($arg)+);
        std::process::exit(1);
    }
}

pub fn init() -> Result<(), log::SetLoggerError> {
    log::set_logger(|max_level| {
        max_level.set(log::LogLevelFilter::Debug);
        Box::new(ConsoleLogger)
    })
}

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &log::LogMetadata) -> bool {
        metadata.level() <= log::LogLevel::Debug
    }

    fn log(&self, record: &log::LogRecord) {
        if self.enabled(record.metadata()) {
            match record.level() {
                log::LogLevel::Error => {
                    console_err!("{} - {}", record.level(), record.args());
                }
                _ => {
                    println!("{} - {}", record.level(), record.args());
                }
            }
        }
    }
}
