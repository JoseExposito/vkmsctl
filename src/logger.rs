use log::{Level, Metadata, Record};
use log::{LevelFilter, SetLoggerError};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init(verbose: bool) -> Result<(), SetLoggerError> {
    let max_level = match verbose {
        true => LevelFilter::Debug,
        false => LevelFilter::Info,
    };

    log::set_logger(&LOGGER).map(|()| log::set_max_level(max_level))
}
