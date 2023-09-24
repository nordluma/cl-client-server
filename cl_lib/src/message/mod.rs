use std::path::PathBuf;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub cmd: String,
    pub cwd: PathBuf,
    pub created_at: DateTime<Local>,
    pub modified_at: DateTime<Local>,
}

impl Payload {
    pub fn new(cmd: String, cwd: PathBuf) -> Payload {
        let now = Local::now();

        Payload {
            cmd,
            cwd,
            created_at: now,
            modified_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    task: Payload,
    success: bool,
    error_message: Option<String>,
}