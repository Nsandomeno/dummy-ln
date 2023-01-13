use time::OffsetDateTime;
use std::fmt;
use lightning::util::logger::{Logger, Record};

struct DLogger();

impl Logger for DLogger {
    fn log(&self, record: &Record) {
        let raw_log = record.args.to_string();
        let log = format!(
            "{} {:<5} [{}:{}] {}\n",
            OffsetDateTime::now_utc(),
            record.level.to_string(),
            record.module_path,
            record.line,
            raw_log
        );
        // Insert code to print this log and/or write this log to disk.
    }
}