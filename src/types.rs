extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Serialize, Deserialize, Debug)]
pub struct LogEntry {
    pub(crate) file: String,
    pub(crate) line: u32,
    pub(crate) message: String,
    pub(crate) severity: String,
    pub(crate) timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogEntries {
    pub(crate) entries: Vec<LogEntry>,
}