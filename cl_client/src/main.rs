use clap::Parser;

use cl_client::cli::args::{Cli, Command};
use cl_lib::network::{init_client_stream, receive_response, send_message};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // 1. Read configurations

    // 2. Parse arguments
    let opts = Cli::parse();

    // 3. Start TCP server

    // 4. Send task
    if let Err(e) = send_task(opts.command).await {
        eprintln!("{}", e);
    };
}

async fn send_task(cmd: Command) -> anyhow::Result<()> {
    let mut sender = init_client_stream("127.0.0.1", "42069").await.unwrap();

    send_message(cmd, &mut sender).await.unwrap();

    let res = receive_response(&mut sender).await.unwrap();

    println!("{:?}", res);

    Ok(())
}
