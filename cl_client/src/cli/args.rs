use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use serde::Serialize;

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
