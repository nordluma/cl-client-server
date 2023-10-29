use ciborium::into_writer;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::message::Message;

use super::GenericStream;

const PACKET_SIZE: usize = 1500;

pub async fn send_message<T>(
    message: T,
    stream: &mut GenericStream,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Into<Message>,
{
    let mut buf = vec![];
    let message: Message = message.into();
    into_writer::<Message, _>(&message, &mut buf)?;

    write_bytes(&buf, stream).await
}

pub async fn write_bytes(
    bytes: &[u8],
    stream: &mut GenericStream,
) -> Result<(), Box<dyn std::error::Error>> {
    let payload_length = bytes.len() as u32;

    // TODO: add error handling
    stream.write_u32(payload_length).await?;

    for chunk in bytes.chunks(PACKET_SIZE) {
        // TODO: add error handling
        stream.write_all(chunk).await?;
    }

    Ok(())
}

pub async fn read_bytes(stream: &mut GenericStream) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // TODO: add error handling
    let payload_length = stream.read_u32().await? as usize;
    let mut payload_bytes = Vec::with_capacity(payload_length);

    while payload_bytes.len() < payload_length {
        let remaining_bytes = payload_length - payload_bytes.len();
        let mut chunk: Vec<u8> = if remaining_bytes < PACKET_SIZE {
            vec![0; remaining_bytes]
        } else {
            vec![0; PACKET_SIZE]
        };

        // TODO: add error handling
        let received_bytes = stream.read_exact(&mut chunk).await?;

        if received_bytes == 0 {
            return Err("Lost connection to client".into());
        }

        payload_bytes.append(&mut chunk);
    }

    Ok(payload_bytes)
}
