use tokio::net::{TcpListener, TcpStream};

type GenericStream = Box<TcpStream>;

pub async fn init_listener(
    settings: (&str, &str),
) -> Result<GenericStream, Box<dyn std::error::Error>> {
    let (host, port) = settings;
    let listener = TcpListener::bind(format!("{}:{}", host, port)).await?;

    Ok(Box::new(listener))
}
