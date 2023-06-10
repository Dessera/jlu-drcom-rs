use crate::utils::{
  config::ConfigStore,
  error::{DrResult, DrcomError},
};
use log::error;
use rand::random;
use tokio::net::UdpSocket;

/// Implementation of Ichallenge,
/// Generate challenge data and send it to server
///
#[derive(Default)]
pub struct ChallengeGenerator {
  pub try_times: u8,
}

impl ChallengeGenerator {
  pub async fn challenge(&mut self, socket: &mut UdpSocket) -> DrResult<()> {
    let mut config = ConfigStore::get_instance()?;
    while self.try_times < 5 {
      // get & send challenge data
      let data = self.get_challenge_data()?;
      socket.send(&data).await?;

      // receive challenge data
      let mut buf = [0; 1024];
      socket.recv(&mut buf).await?;

      if buf[0] == 0x02 {
        config.salt = [buf[4], buf[5], buf[6], buf[7]];
        config.client_ip = [buf[20], buf[21], buf[22], buf[23]];
        return Ok(());
      }

      error!("Challenge failed, try again");
      self.try_times += 1;
    }
    Err(DrcomError::ChallengeError(
      "challenge times out".to_string(),
    ))
  }

  fn get_challenge_data(&self) -> DrResult<Vec<u8>> {
    let mut data = vec![0x01, 0x02 + self.try_times, random(), random(), 0x6a];
    data.resize(20, 0x00);
    Ok(data)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_challenge() {
    simple_logger::init().unwrap();
    ConfigStore::init().unwrap();

    let mut generator = ChallengeGenerator::default();
    let mut socket = tokio::net::UdpSocket::bind("0.0.0.0:0").await.unwrap();
    socket.connect("10.100.61.3:61440").await.unwrap();
    let result = generator.challenge(&mut socket).await;
    assert!(result.is_ok());
  }
}
