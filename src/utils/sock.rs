use tokio::net::UdpSocket;
use tokio::time::Duration;

use crate::utils::error::DrcomError;

use super::error::DrResult;

pub struct DrSocket {
  socket: UdpSocket,
  timeout: Duration,
}

impl DrSocket {
  pub fn new(socket: UdpSocket) -> Self {
    Self {
      socket,
      timeout: Duration::from_secs(5),
    }
  }

  pub async fn send(&mut self, buf: &[u8]) -> DrResult<()> {
    self.socket.send(buf).await?;
    Ok(())
  }

  pub async fn recv(&mut self, buf: &mut [u8]) -> DrResult<()> {
    self.socket.recv(buf).await?;
    Ok(())
  }

  pub async fn recv_with_timeout(&mut self, buf: &mut [u8]) -> DrResult<()> {
    tokio::select! {
        res = self.socket.recv(buf) => {
          res?;
        },
        _ = tokio::time::sleep(self.timeout) => {
          return Err(DrcomError::SocketError("recv timeout".to_string()));
      }
    }
    Ok(())
  }
}
