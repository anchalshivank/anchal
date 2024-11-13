use anchal::protocols::tcp::client::TcpClient;
use anchal::transport::{Transport, TransportClient};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Initialize the client
    let mut client = TcpClient::new();

    // Connect to the server
    client.connect(addr).await.unwrap();
    println!("Connected to the server at {}", addr);

    // Send a message to the server
    let data = b"Hello from client";
    client.stream().send(data).await.unwrap();
    println!("Sent data to server: {}", String::from_utf8_lossy(data));

    // Receive a response from the server
    let response = client.stream().receive().await.unwrap();
    println!(
        "Received from server: {}",
        String::from_utf8_lossy(&response)
    );
}
