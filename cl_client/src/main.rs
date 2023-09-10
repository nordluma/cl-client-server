use anyhow::{Context, Result};
use ciborium::into_writer;
use clap::Parser;
use tokio::{io::AsyncWriteExt, net::TcpStream};

use cl_client::cli::args::{Cli, Command};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Parse arguments
    let opts = Cli::parse();

    if let Err(e) = send_task(opts.command).await {
        eprintln!("{}", e);
    };
}

async fn send_task(cmd: Command) -> Result<()> {
    let mut sender = TcpStream::connect("127.0.0.1:42069")
        .await
        .context("could not connect to server")?;

    let msg = to_cbor(&cmd).await?;

    sender
        .write_all(&msg)
        .await
        .context("could not send the message")?;

    Ok(())
}

async fn to_cbor(cmd: &Command) -> anyhow::Result<Vec<u8>> {
    let mut buf = vec![];
    into_writer(&cmd, &mut buf).context("could not serialize task")?;

    Ok(buf)
}
