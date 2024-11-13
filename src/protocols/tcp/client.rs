use crate::error::TransportError;
use crate::protocols::tcp::connection::TcpTransportStream;
use crate::transport::{Transport, TransportClient, TransportStream};
use async_trait::async_trait;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct TcpClient {
    connection: Option<TcpTransportStream>,
}

impl TcpClient {
    // Initialize a new TcpClient with no active connection
    pub fn new() -> Self {
        Self { connection: None }
    }
}

#[async_trait]
impl Transport for TcpClient {
    async fn connect(&mut self, addr: SocketAddr) -> Result<(), TransportError> {
        // Attempt to connect to the specified address
        let stream = TcpStream::connect(addr).await.map_err(|e| {
            TransportError::ConnectionError(format!("Failed to connect to {}: {}", addr, e))
        })?;

        // Wrap the TcpStream in a TcpTransportStream and store it in `connection`
        self.connection = Some(TcpTransportStream::new(stream));
        println!("Connected to {}", addr);

        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), TransportError> {
        // Check if there is an active connection to disconnect
        if let Some(ref mut conn) = self.connection {
            // Shutdown the stream gracefully
            conn.stream.shutdown().await.map_err(|e| {
                TransportError::DisconnectError(format!("Failed to disconnect: {}", e))
            })?;
            println!("Disconnected from the server");
        } else {
            return Err(TransportError::DisconnectError(
                "No active connection".to_string(),
            ));
        }

        // Set the connection to None to mark it as disconnected
        self.connection = None;
        Ok(())
    }
}

#[async_trait]
impl TransportClient for TcpClient {
    fn stream(&mut self) -> &mut dyn TransportStream {
        // Return a mutable reference to the `TcpTransportStream` for data transmission
        self.connection
            .as_mut()
            .expect("Connection not established")
    }
}
