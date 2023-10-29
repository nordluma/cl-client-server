use tokio::net::{TcpListener, TcpStream};

pub type GenericStream = Box<TcpStream>;
pub type Listener = Box<TcpListener>;

pub async fn init_listener(settings: (&str, &str)) -> Result<Listener, Box<dyn std::error::Error>> {
    let (host, port) = settings;
    let listener = TcpListener::bind(format!("{}:{}", host, port)).await?;

    Ok(Box::new(listener))
}
