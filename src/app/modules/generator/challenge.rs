use rand::random;

use crate::app::utils::config::ConfigStore;
use crate::app::utils::error::DrResult;
use crate::app::utils::error::DrcomError;
use crate::app::utils::interface::Ichallenge;
use log::{error, info};

pub struct ChallengeGenerator {
  pub try_times: u8,
}

impl Ichallenge for ChallengeGenerator {
  fn challenge(&mut self, socket: &mut std::net::UdpSocket) -> DrResult<()> {
    info!("start data challenge");
    while self.try_times < 5 {
      info!("send challenge data: try times {}", self.try_times);
      // get & send challenge data
      let data = self.get_challenge_data();
      socket.send(&data)?;

      // receive challenge data
      let mut buf = [0; 1024];
      socket.recv(&mut buf)?;

      if buf[0] == 0x02 {
        ConfigStore::get_instance().lock().unwrap().salt = [buf[4], buf[5], buf[6], buf[7]];
        info!("challenge success!");
        return Ok(());
      }

      error!("challenge failed, try again");
      self.try_times += 1;
    }
    Err(DrcomError::ChallengeError(
      "challenge times out".to_string(),
    ))
  }

  fn get_challenge_data(&self) -> Vec<u8> {
    let mut data = vec![0x01, 0x02 + self.try_times, random(), random(), 0x6a];
    data.resize(20, 0x00);
    data
  }
}

impl Default for ChallengeGenerator {
  fn default() -> Self {
    Self::new()
  }
}

impl ChallengeGenerator {
  pub fn new() -> Self {
    Self { try_times: 0 }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_challenge() {
    simple_logger::init().unwrap();
    ConfigStore::init();

    let mut generator = ChallengeGenerator::new();
    let mut socket = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect("10.100.61.3:61440").unwrap();
    let result = generator.challenge(&mut socket);
    assert!(result.is_ok());
  }
}
