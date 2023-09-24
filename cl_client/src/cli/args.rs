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

impl Into<Payload> for Task {
    fn into(self) -> Payload {
        if self.cwd.is_some() {
            Payload::new(self.cmd, self.cwd.unwrap())
        } else {
            Payload::new(self.cmd, PathBuf::from("./"))
        }
    }
}
