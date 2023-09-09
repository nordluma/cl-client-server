use std::path::PathBuf;

use anyhow::{Context, Result};
use ciborium::from_reader;
use serde::Deserialize;
use tokio::{
    io::{AsyncRead, AsyncReadExt},
    net::{TcpListener, TcpStream},
};

#[derive(Debug, Deserialize)]
enum Command {
    Add(Task),
    Run,
    Kill,
    Show,
}

#[derive(Debug, Deserialize)]
struct Task {
    cmd: String,
    cwd: Option<PathBuf>,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:42069").await?;

    let (socket, _) = listener
        .accept()
        .await
        .context("Could not get the client")?;

    receive_task(socket).await?;

    Ok(())
}

async fn receive_task(stream: TcpStream) -> Result<()> {
    let cmd = read_bytes(stream)?;

    match cmd {
        Command::Add(task) => {
            println!("Addign task: {:?}", task);
        }
        Command::Run => {
            println!("Running all tasks");
        }
        Command::Kill => {
            println!("Killing task");
        }
        Command::Show => {
            println!("showing tasks");
        }
    }

    Ok(())
}

fn read_bytes(stream: TcpStream) -> Result<Command> {
    // This feels wrong...
    // There should be a way to deserialize without turning the stream into
    // a std stream
    let std_stream = stream.into_std().unwrap();
    let cmd = from_reader::<Command, _>(std_stream).unwrap();

    Ok(cmd)
}
