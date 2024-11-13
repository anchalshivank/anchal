use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransportError {
    #[error("Binding error: {0}")]
    BindingError(String),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Disconnect error: {0}")]
    DisconnectError(String),

    #[error("Send error: {0}")]
    SendError(String),

    #[error("Receive error: {0}")]
    ReceiveError(String),

    #[error("Unknown error")]
    Unknown,
}
