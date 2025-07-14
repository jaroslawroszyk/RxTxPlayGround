use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::mpsc::{Receiver, Sender};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub content: String,
}

pub struct Endpoint {
    pub rx: Receiver<Message>,
    pub tx: Sender<Message>,
}

impl Endpoint {
    pub fn new(rx: Receiver<Message>, tx: Sender<Message>) -> Self {
        Self { rx, tx }
    }

    pub fn from_tcp_stream(stream: TcpStream) -> Self {
        let (tx_to_endpoint, rx_from_tcp) = tokio::sync::mpsc::channel(100);
        let (tx_to_tcp, rx_from_endpoint) = tokio::sync::mpsc::channel(100);

        let (read_half, write_half) = stream.into_split();

        tokio::spawn(async move {
            let mut read_half = read_half;
            loop {
                match read_message(&mut read_half).await {
                    Ok(Some(msg)) => {
                        if tx_to_endpoint.send(msg).await.is_err() {
                            break;
                        }
                    }
                    Ok(None) => break,
                    Err(e) => {
                        eprintln!("Error reading from TCP: {e}");
                        break;
                    }
                }
            }
        });

        tokio::spawn(async move {
            let mut rx = rx_from_endpoint;
            let mut write_half = write_half;

            while let Some(msg) = rx.recv().await {
                if let Err(e) = write_message(&mut write_half, &msg).await {
                    eprintln!("Error writing to TCP: {e}");
                    break;
                }
            }
        });

        Self {
            rx: rx_from_tcp,
            tx: tx_to_tcp,
        }
    }

    pub async fn send(&self, msg: Message) {
        let _ = self.tx.send(msg).await;
    }

    pub async fn receive(&mut self) -> Option<Message> {
        self.rx.recv().await
    }
}

async fn read_message(
    stream: &mut OwnedReadHalf,
) -> Result<Option<Message>, Box<dyn std::error::Error + Send + Sync>> {
    let mut len_bytes = [0u8; 4];
    if stream.read_exact(&mut len_bytes).await.is_err() {
        return Ok(None);
    }

    let len = u32::from_be_bytes(len_bytes) as usize;

    let mut buffer = vec![0u8; len];
    stream.read_exact(&mut buffer).await?;

    let json = String::from_utf8(buffer)?;
    let msg: Message = serde_json::from_str(&json)?;
    Ok(Some(msg))
}

async fn write_message(
    stream: &mut OwnedWriteHalf,
    msg: &Message,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let json = serde_json::to_string(msg)?;
    let len = json.len() as u32;

    stream.write_all(&len.to_be_bytes()).await?;
    stream.write_all(json.as_bytes()).await?;
    Ok(())
}
