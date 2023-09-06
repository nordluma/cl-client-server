// Parse arguments
use clap::Parser;

use cl_client::cli::args::Cli;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let opts = Cli::parse();

    println!("{:?}", opts)
}
