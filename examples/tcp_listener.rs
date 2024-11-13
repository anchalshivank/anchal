use anchal::protocols::tcp::server::TcpListener;
use anchal::transport::TransportListener;
use std::net::SocketAddr;
use tokio::{signal, task};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Initialize the listener
    let mut listener = TcpListener::new(addr).await.unwrap();

    // Run the listener to start accepting connections
    task::spawn(async move {
        if let Err(e) = listener.listen(addr).await {
            eprintln!("Failed to start listener: {}", e);
        }
    });

    println!("Server is listening on {}", addr);

    // Keep the main function alive to ensure the server keeps running
    signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl_c signal");

    println!("Server shutting down.");
}
