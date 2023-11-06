use tokio::net::{TcpListener, TcpStream};

use crate::error::ClError;

pub type GenericStream = Box<TcpStream>;
pub type Listener = Box<TcpListener>;

pub async fn init_listener(settings: (&str, &str)) -> Result<Listener, ClError> {
    let (host, port) = settings;
    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .map_err(|err| ClError::ConnectionError(format!("Could not start listener: {}", err)))?;

    Ok(Box::new(listener))
}

pub async fn init_client_stream(host: &str, port: &str) -> Result<GenericStream, ClError> {
    let sender = TcpStream::connect(format!("{}:{}", host, port))
        .await
        .map_err(|err| {
            ClError::ConnectionError(format!(
                "Could not establish connection with server: {}",
                err
            ))
        })?;

    Ok(Box::new(sender))
}
