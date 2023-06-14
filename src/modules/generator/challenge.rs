use crate::utils::{
  config::ConfigStore,
  error::{DrResult, DrcomError},
  sock::DrSocket,
};
use log::error;
use rand::random;

#[derive(Default)]
pub struct ChallengeGenerator {
  pub try_times: u8,
}

impl ChallengeGenerator {
  pub async fn challenge(&mut self, socket: &mut DrSocket) -> DrResult<()> {
    let mut config = ConfigStore::get_instance()?;
    while self.try_times < 5 {
      // get & send challenge data
      let data = self.get_challenge_data()?;
      socket.send(&data).await?;

      // receive challenge data
      let mut buf = [0; 1024];
      socket.recv_with_timeout(&mut buf).await?;

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
    let mut data = vec![0x00; 20];
    data[0] = 0x01;
    data[1] = 0x02 + self.try_times;
    data[2] = random();
    data[3] = random();
    data[4] = 0x6a;
    Ok(data)
  }
}

#[cfg(test)]
mod tests {
  use std::time::Duration;

  use simplelog::SimpleLogger;

  use super::*;

  #[tokio::test]
  async fn test_challenge() {
    SimpleLogger::init(log::LevelFilter::Trace, simplelog::Config::default()).unwrap();
    ConfigStore::init().unwrap();

    let mut generator = ChallengeGenerator::default();
    let mut socket = DrSocket::create("0.0.0.0:0", Duration::from_secs(5))
      .await
      .unwrap();
    let result = generator.challenge(&mut socket).await;
    assert!(result.is_ok());
  }
}
