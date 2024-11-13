use crate::error::TransportError;
use crate::protocols::tcp::connection::TcpTransportStream;
use crate::transport::{TransportListener, TransportStream};
use async_trait::async_trait;
use std::net::SocketAddr;
use tokio::net::TcpListener as TokioTcpListener;
use tokio::task;

pub struct TcpListener {
    listener: Option<TokioTcpListener>,
}

impl TcpListener {
    pub async fn new(addr: SocketAddr) -> Result<Self, TransportError> {
        // Create a new Tokio TcpListener bound to the specified address
        let listener = TokioTcpListener::bind(addr)
            .await
            .map_err(|err| TransportError::BindingError(err.to_string()))?;
        Ok(Self {
            listener: Some(listener),
        })
    }
}

#[async_trait]
impl TransportListener for TcpListener {
    async fn listen(&mut self, addr: SocketAddr) -> Result<(), TransportError> {
        // Ensure the listener is initialized
        let listener = self.listener.take().ok_or_else(|| {
            TransportError::ConnectionError("Listener not initialized".to_string())
        })?;

        println!("TCP listener started at {}", addr);

        loop {
            // Accept incoming connections
            match listener.accept().await {
                Ok((stream, _addr)) => {
                    // Wrap TcpStream in TcpTransportStream (which implements TransportStream)
                    let connection = TcpTransportStream::new(stream);

                    // Spawn a new task to handle the connection concurrently
                    task::spawn(async move {
                        if let Err(err) = Self::handle_connection(Box::new(connection)).await {
                            eprintln!("Error handling connection: {}", err);
                        }
                    });
                }
                Err(err) => {
                    eprintln!("Error while accepting connection: {}", err);
                    return Err(TransportError::ReceiveError(err.to_string()));
                }
            }
        }
    }

    async fn close(&self) -> Result<(), TransportError> {
        println!("Closing TCP listener");
        // No explicit action required to close the Tokio listener
        Ok(())
    }

    async fn handle_connection(mut stream: Box<dyn TransportStream>) -> Result<(), TransportError> {
        // Example: Echo back any received data
        if let Ok(data) = stream.receive().await {
            println!("Received data: {:?}", data);

            // Send a response back to the client
            stream.send(b"Hello from server").await?;
            println!("Sent response to client");
        }
        Ok(())
    }
}
