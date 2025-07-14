use std::error::Error;
use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let socket = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connected to server");

    let (mut reader, mut writer) = socket.into_split();
    let stdin = BufReader::new(io::stdin());
    let mut lines = stdin.lines();

    writer.write_all(b"hello from client\n").await?;

    let read_task = tokio::spawn(async move {
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
        writer.write_all(data.as_bytes()).await?;
    }

    read_task.await?;
    Ok(())
}
