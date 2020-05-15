mod asl;

pub use self::asl::*;
pub use asl_derive::*;

use log::{LevelFilter, Log, Metadata, Record};
use std::sync::Once;
static LOG_INITIALIZED: Once = Once::new();
static LOGGER: Logger = Logger;

struct Logger;

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            asl::print_message(&format!("[{}] {}", record.level(), record.args()));
        }
    }

    fn flush(&self) {}
}

pub fn init_log() {
    LOG_INITIALIZED.call_once(|| {
        log::set_logger(&LOGGER).unwrap();
        log::set_max_level(LevelFilter::Info);
    });
}
