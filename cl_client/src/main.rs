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

    // 3. Create a client
    let mut client = Client::new(&config.network).await?;

    // 4. Start TCP server

    // 5. Send task
    if let Err(e) = client.send_task(opts.command).await {
        eprintln!("{}", e);
    };

    Ok(())
}

struct Client {
    stream: GenericStream,
}

impl Client {
    async fn new(config: &NetworkSettings) -> anyhow::Result<Self> {
        let sender = init_client_stream(&config.addr, &config.port).await?;

        Ok(Self { stream: sender })
    }

    async fn send_task(&mut self, cmd: Command) -> anyhow::Result<()> {
        send_message(cmd, &mut self.stream).await?;

        match receive_response(&mut self.stream).await? {
            Response::Success(res) | Response::Failure(res) => {
                println!("{}", res)
            }
            Response::Status(_) => unimplemented!(),
            Response::EmptyResponse => {}
        }

        Ok(())
    }
}
