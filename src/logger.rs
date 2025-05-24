use colored::Colorize;
use log::{Level, Metadata, Record};

pub struct EoLogger;

impl log::Log for EoLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if record.level() == Level::Info {
            println!("[ {} ]: {}", record.level().to_string().to_lowercase().green(), record.args());
        }
        if record.level() == Level::Debug {
            println!("[ {} ]: {}", record.level().to_string().to_lowercase().bright_yellow(), record.args());
        }
        if record.level() == Level::Warn {
            println!("[ {} ]: {}", record.level().to_string().to_lowercase().yellow(), record.args());
        }
        if record.level() == Level::Error {
            println!("[ {} ]: {}", record.level().to_string().to_lowercase().red(), record.args());
        }
        if record.level() == Level::Trace {
            println!("[ {} ]: {}", record.level().to_string().to_lowercase().bright_purple(), record.args());
        }
    }

    fn flush(&self) {

    }
}