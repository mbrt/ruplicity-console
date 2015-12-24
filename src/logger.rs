#![macro_use]

use std::io::{self, Write};
use log;

macro_rules! fatal {
    ($($arg:tt)+) => {
        error!($($arg)+);
        std::process::exit(1);
    }
}

pub fn init(level: log::LogLevelFilter) -> Result<(), log::SetLoggerError> {
    log::set_logger(|max_level| {
        max_level.set(level);
        Box::new(ConsoleLogger)
    })
}

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, _metadata: &log::LogMetadata) -> bool {
        true
    }

    fn log(&self, record: &log::LogRecord) {
        match record.level() {
            log::LogLevel::Error => {
                console_err!(target: io::stderr(), "{} - {}", record.level(), record.args());
            }
            log::LogLevel::Warn => {
                console_warn!(target: io::stderr(), "{} - {}", record.level(), record.args());
            }
            _ => {
                console_info!(target: io::stderr(), "{} - {}", record.level(), record.args());
            }
        }
    }
}
