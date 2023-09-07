mod types;

extern crate reqwest;

use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead};
use std::thread::sleep;
use std::time::Duration;
use crate::types::LogEntries;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub const LOG_FLUSH_BOUNDARY: usize = 10_000;
pub const LOG_REMAINS: usize = 2_000;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Initialize an empty queue
    let mut log_queue: VecDeque<String> = VecDeque::new();
    // TODO Could use a hash of the string to improve memory
    let mut current_log_entries: HashSet<String> = HashSet::new();


    loop {
        if log_queue.len() > LOG_FLUSH_BOUNDARY {
            flush_logs(&mut log_queue, &mut current_log_entries);
        } else {
            pull_logs(&mut log_queue, &mut current_log_entries).await?;
        }

        println!("Sleeping...");
        sleep(Duration::from_secs(10));
    }
}

fn flush_logs(log_queue: &mut VecDeque<String>, current_log_entries: &mut HashSet<String>) {
    // TODO could append once with a long string. Trouble is removing the values from the memory queue
    // and keeping them if the write fails
    for _ in 0..LOG_FLUSH_BOUNDARY - LOG_REMAINS {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("logfile.txt")
            .unwrap();


        let log = log_queue.front().unwrap().clone();
        writeln!(file, "{}", log).unwrap();

        log_queue.pop_front();
        current_log_entries.remove(&log);
    }
}

async fn pull_logs(log_queue: &mut VecDeque<String>, current_log_entries: &mut HashSet<String>) -> Result<(), reqwest::Error>{
    // The HTTPS endpoint to pull from
    let url = "https://uc3qt-6yaaa-aaaaq-aabnq-cai.raw.icp0.io/logs";

    // Make a GET request
    let resp = reqwest::get(url).await?;

    // Read the response line-by-line
    let binding = resp.text().await?;
    let mut reader = io::BufReader::new(binding.as_bytes());

    let mut buf = "".to_string();
    let size = reader.read_line(&mut buf);
    assert!(size.unwrap() > 0);

    // Deserialize the JSON string into LogEntries struct
    let log_entries: LogEntries = serde_json::from_str(&buf).unwrap();

    for log_entry in log_entries.entries {
        let stringified = serde_json::to_string(&log_entry).unwrap();

        // Already processed this log
        if current_log_entries.contains(&stringified) {
            continue;
        }

        println!("{}", stringified);
        log_queue.push_back(stringified.clone());
        current_log_entries.insert(stringified.clone());
    }

    Ok(())
}