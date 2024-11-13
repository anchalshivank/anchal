use crate::error::TransportError;
use crate::transport::{ReceiveStream, SendStream, TransportStream};
use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct TcpTransportStream {
    pub(crate) stream: TcpStream,
}

impl TcpTransportStream {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }
}

#[async_trait]
impl SendStream for TcpTransportStream {
    async fn send(&mut self, data: &[u8]) -> Result<usize, TransportError> {
        // Send the provided data over the TCP stream
        self.stream
            .write_all(data)
            .await
            .map_err(|err| TransportError::SendError(err.to_string()))?;
        Ok(data.len())
    }
}

#[async_trait]
impl ReceiveStream for TcpTransportStream {
    async fn receive(&mut self) -> Result<Vec<u8>, TransportError> {
        let mut buffer = vec![0; 1024];
        // Read data into the buffer and return the result
        let n = self
            .stream
            .read(&mut buffer)
            .await
            .map_err(|err| TransportError::ReceiveError(err.to_string()))?;
        buffer.truncate(n); // Trim the buffer to the actual received data length
        Ok(buffer)
    }
}

#[async_trait]
impl TransportStream for TcpTransportStream {}
