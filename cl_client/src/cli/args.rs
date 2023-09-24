use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use serde::Serialize;

use cl_lib::message::Payload;

#[derive(Debug, Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand, Serialize)]
pub enum Command {
    Add(Task),
    Run,
    Kill,
    Show,
}

#[derive(Args, Debug, Serialize)]
pub struct Task {
    pub cmd: String,
    pub cwd: Option<PathBuf>,
}

impl From<Task> for Payload {
    fn from(val: Task) -> Self {
        if val.cwd.is_some() {
            Payload::new(val.cmd, val.cwd.unwrap())
        } else {
            Payload::new(val.cmd, PathBuf::from("./"))
        }
    }
}
