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
    }

    fn flush(&self) {

    }
}