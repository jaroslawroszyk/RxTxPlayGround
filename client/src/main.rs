use std::error::Error;
use std::sync::Arc;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let socket = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connected to server");

    let socket = Arc::new(Mutex::new(socket));
    let stdin = BufReader::new(io::stdin());
    let mut lines = stdin.lines();

    let reader_socket = Arc::clone(&socket);
    let read_task = tokio::spawn(async move {
        let mut reader = reader_socket.lock().await;
        let mut buf = vec![0u8; 1024];
        loop {
            let n = reader.read(&mut buf).await.unwrap_or(0);
            if n == 0 {
                break;
            }
            print!("Received: {}", String::from_utf8_lossy(&buf[..n]));
        }
    });

    while let Some(line) = lines.next_line().await? {
        let data = line + "\n";
        let mut writer = socket.lock().await;
        writer.write_all(data.as_bytes()).await?;
    }

    read_task.await?;

    Ok(())
}
