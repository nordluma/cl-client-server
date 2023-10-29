use ciborium::into_writer;
use tokio::io::AsyncWriteExt;

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
