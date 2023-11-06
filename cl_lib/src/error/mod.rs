use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClError {
    #[error("{0}")]
    IOError(io::Error),
    #[error("Error with connection: {0}")]
    ConnectionError(String),
    #[error("Could not deserialize: {0}")]
    DeserializationError(#[source] anyhow::Error),
    #[error("Could not serialize: {0}")]
    SerializationError(#[source] anyhow::Error),
}
