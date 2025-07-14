use endpoints::endpoints::{Endpoint, Message};
use std::error::Error;
use tokio::net::{TcpListener, TcpStream};

async fn handle_client(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut endpoint = Endpoint::from_tcp_stream(stream);

    while let Some(msg) = endpoint.receive().await {
        println!("Server received: {msg:?}");

        let response = Message {
            content: "Hi client!".to_string(),
        };

        endpoint.send(response).await;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind to address");
    println!("Server listening on 127.0.0.1:8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New client: {addr}");

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("Error handling client {addr}: {e:?}");
            }
            println!("Client {addr} disconnected");
        });
    }
}
