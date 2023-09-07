use anyhow::{Context, Result};
use byteorder::{NetworkEndian, WriteBytesExt};
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
    // send the task to the server
    let mut buf = Vec::new();
    let mut sender = TcpStream::connect("127.0.0.1:42069")
        .await
        .context("could not connect to server")?;

    into_writer(&cmd, &mut buf).context("could not serialize message")?;
    let header = buf.len() as u64;

    WriteBytesExt::write_u64::<NetworkEndian>(&mut buf, header)?;

    println!("{:?}", buf);

    sender
        .write_all(&buf)
        .await
        .context("could not send the message")?;
    // receive response
    Ok(())
}
