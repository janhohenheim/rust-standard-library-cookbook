#[macro_use]
extern crate log;

use log::{Level, Metadata, Record};
use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::{error, fmt, result};
use std::sync::RwLock;

struct FileLogger {
    log_level: Level,
    writer: RwLock<BufWriter<File>>,
}

impl log::Log for FileLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.log_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut writer = self.writer.write().expect("To do: Add error message");
            write!(writer, "ayy").expect("To do: Add error message");
        }
    }

    fn flush(&self) {
        // To do: Find out if and when we need this
        self.writer
            .write()
            .expect("To do: Add error message")
            .flush()
            .expect("To do: Add error message");
    }
}

impl FileLogger {
    // A convenience method to set everything up nicely
    fn init(log_level: Level) -> Result<()> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("log.txt")?;
        let writer = RwLock::new(BufWriter::new(file));
        let logger = FileLogger { log_level, writer };
        log::set_boxed_logger(Box::new(logger))?;
        Ok(())
    }
}

// Our custom error for our logger
#[derive(Debug)]
enum FileLoggerError {
    Io(io::Error),
    SetLogger(log::SetLoggerError),
}

type Result<T> = result::Result<T, FileLoggerError>;
impl error::Error for FileLoggerError {
    fn description(&self) -> &str {
        match *self {
            FileLoggerError::Io(ref err) => err.description(),
            FileLoggerError::SetLogger(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            FileLoggerError::Io(ref err) => Some(err),
            FileLoggerError::SetLogger(ref err) => Some(err),
        }
    }
}

impl fmt::Display for FileLoggerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileLoggerError::Io(ref err) => write!(f, "IO error: {}", err),
            FileLoggerError::SetLogger(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl From<io::Error> for FileLoggerError {
    fn from(err: io::Error) -> FileLoggerError {
        FileLoggerError::Io(err)
    }
}

impl From<log::SetLoggerError> for FileLoggerError {
    fn from(err: log::SetLoggerError) -> FileLoggerError {
        FileLoggerError::SetLogger(err)
    }
}


fn main() {
    FileLogger::init(Level::Warn).expect("Failed to init FileLogger");
    warn!("ayy");
}
