use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Add(Task),
    Run,
    Kill,
    Show,
}

#[derive(Args, Debug)]
pub struct Task {
    pub cmd: String,
    pub cwd: PathBuf,
}
