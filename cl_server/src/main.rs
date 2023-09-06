use anyhow::{Context, Result};
use tokio::net::TcpListener;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:42069").await?;

    let (socket, _) = listener
        .accept()
        .await
        .context("Could not get the client")?;

    Ok(())
}
