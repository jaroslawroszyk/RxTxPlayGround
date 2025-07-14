use std::error::Error;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

struct UppercaseEndpoint;

impl UppercaseEndpoint {
    fn process_in_place(buffer: &mut [u8], n: usize) {
        for byte in &mut buffer[..n] {
            if b'a' <= *byte && *byte <= b'z' {
                *byte -= 32;
            }
        }
    }
}

async fn handle_client(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf = [0u8; 1024];

    loop {
        let n = socket.read(&mut buf).await?;
        if let Ok(msg) = std::str::from_utf8(&buf[..n]) {
            println!("Received message: {msg}");
        } else {
            println!("Received non-UTF8 data: {:?}", &buf[..n]);
        }
        if n == 0 {
            break;
        }

        UppercaseEndpoint::process_in_place(&mut buf, n);

        socket.write_all(&buf[..n]).await?;
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
