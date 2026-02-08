// TCP Client (Phase 3.2.1, TDD).

use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Debug, Error)]
pub enum TCPClientError {
    #[error("IO: {0}")]
    Io(#[from] std::io::Error),
}

const RECV_BUF_SIZE: usize = 4096;

pub struct TCPClient {
    stream: TcpStream,
}

impl TCPClient {
    pub async fn connect(host: &str, port: u16) -> Result<Self, TCPClientError> {
        let addr = format!("{}:{}", host, port);
        let stream = TcpStream::connect(&addr).await?;
        Ok(Self { stream })
    }

    pub async fn send(&mut self, data: &[u8]) -> Result<(), TCPClientError> {
        self.stream.write_all(data).await?;
        Ok(())
    }

    pub async fn receive(&mut self) -> Result<Vec<u8>, TCPClientError> {
        let mut buf = vec![0u8; RECV_BUF_SIZE];
        let n = self.stream.read(&mut buf).await?;
        Ok(buf[..n].to_vec())
    }
}
