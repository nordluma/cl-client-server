use std::{io::Cursor, path::PathBuf};

use anyhow::{Context, Result};
use ciborium::from_reader;
use serde::Deserialize;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

#[derive(Debug, Deserialize)]
enum Command {
    Add(Task),
    Run,
    Kill,
    Show,
}

#[allow(dead_code)]
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

    let task = receive_task(socket).await?;

    match task {
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

async fn receive_task(stream: TcpStream) -> Result<Command> {
    let bytes = read_bytes(stream).await?;
    let cmd = from_reader::<Command, _>(Cursor::new(bytes)).unwrap();

    Ok(cmd)
}

async fn read_bytes(mut stream: TcpStream) -> Result<Vec<u8>> {
    // This feels wrong...
    // There should be a way to deserialize without turning the stream into
    // a std stream
    let mut buf = vec![];
    stream.read_to_end(&mut buf).await?;
    //let cmd = from_reader::<Command, _>(std_stream).unwrap();

    Ok(buf)
}
