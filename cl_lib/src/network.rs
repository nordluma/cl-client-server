use std::path::PathBuf;

use chrono;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    cmd: String,
    cwd: PathBuf,
    //created_at:
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    task: Message,
    //completed_at:
    success: bool,
}
