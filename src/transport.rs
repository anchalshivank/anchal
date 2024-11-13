use crate::error::TransportError;
use async_trait::async_trait;
use std::net::SocketAddr;

#[async_trait]
pub trait Transport: Send + Sync {
    /// Connect to a specified address.
    async fn connect(&mut self, addr: SocketAddr) -> Result<(), TransportError>;

    /// Disconnect from the current connection.
    async fn disconnect(&mut self) -> Result<(), TransportError>;
}

#[async_trait]
pub trait SendStream: Send + Sync {
    /// Send data to the remote endpoint.
    async fn send(&mut self, data: &[u8]) -> Result<usize, TransportError>;
}

#[async_trait]
pub trait ReceiveStream: Send + Sync {
    /// Receive data from the remote endpoint.
    async fn receive(&mut self) -> Result<Vec<u8>, TransportError>;
}

#[async_trait]
pub trait TransportStream: SendStream + ReceiveStream {}

#[async_trait]
pub trait TransportListener {
    /// Start listening for incoming connections at the specified address.
    async fn listen(&mut self, addr: SocketAddr) -> Result<(), TransportError>;

    /// Close the listener.
    async fn close(&self) -> Result<(), TransportError>;

    /// Handle an accepted connection using the provided TransportStream.
    async fn handle_connection(stream: Box<dyn TransportStream>) -> Result<(), TransportError>;
}

#[async_trait]
pub trait TransportClient: Transport {
    /// Provides access to the TransportStream for bidirectional communication.
    fn stream(&mut self) -> &mut dyn TransportStream;
}
