use std::{error::Error, net::TcpListener};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listening on 127.0.0.1:8080");

    loop {
        let (socket, addr) = listener.accept()?;
        println!("New client: {}", addr);

        //     tokio::spawn(async move {
        //         if let Err(e) = handle_client(socket).await {
        //             eprintln!("Error handling client {}: {:?}", addr, e);
        //         }
        //         println!("Client {} disconnected", addr);
        //     });
    }
}
