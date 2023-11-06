use clap::Parser;

use cl_client::{
    cli::args::{Cli, Command},
    config::{Configurations, NetworkSettings},
};
use cl_lib::{
    message::Response,
    network::{init_client_stream, receive_response, send_message, GenericStream},
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // 1. Parse arguments
    let opts = Cli::parse();

    // 2. Read configurations
    let config = Configurations::build();

    if let Err(e) = send_task(opts.command).await {
    // 3. Create a client

    // 4. Start TCP server

    // 5. Send task
        eprintln!("{}", e);
    };

    Ok(())
}

async fn send_task(cmd: Command) -> anyhow::Result<()> {
    let mut sender = init_client_stream("127.0.0.1", "42069").await?;

    send_message(cmd, &mut sender).await?;

    let res = receive_response(&mut sender).await?;

    println!("{:?}", res);

    Ok(())
}
