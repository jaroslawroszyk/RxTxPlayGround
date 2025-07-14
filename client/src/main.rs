use endpoints::endpoints::{Endpoint, Message};
use std::error::Error;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connected to server");

    let mut client_endpoint = Endpoint::from_tcp_stream(stream);

    let message = Message {
        content: "Hello from client".to_string(),
    };

    client_endpoint.send(message).await;
    println!("Sent message to server");

    if let Some(reply) = client_endpoint.receive().await {
        println!("Client received: {reply:?}");
    } else {
        println!("Server disconnected");
    }

    Ok(())
}
