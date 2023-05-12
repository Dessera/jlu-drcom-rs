use std::net::UdpSocket;

use crate::app::utils::{
  error::{DrResult, DrcomError},
  interface::Ichallenge,
};

/**
 * DrcomConnection， which only satisfies the D version.
 */
pub struct DrcomConnection {
  pub username: String,
  pub password: String,
  pub mac_addr: u64,
  pub destination: String,
  socket: UdpSocket,
  try_times: u8,
  buf: [u8; 1024],
}

impl DrcomConnection {
  pub fn new(username: &str, password: &str, mac: u64) -> Self {
    Self {
      username: username.to_string(),
      password: password.to_string(),
      mac_addr: mac,
      destination: String::from("10.100.61.3:61440"),
      socket: UdpSocket::bind("0.0.0.0:0").unwrap(),
      try_times: 0,
      buf: [0; 1024],
    }
  }
}

impl Ichallenge for DrcomConnection {
  fn challenge(&mut self) -> DrResult<()> {
    while self.try_times < 5 {
      let clg_data = self.get_challenge_data();
      let success = self
        .socket
        .send_to(&clg_data, self.destination.as_str())
        .unwrap();
      if success != clg_data.len() {
        eprintln!(
          "Challenge data send error: challenge data length: {}, send length: {}",
          clg_data.len(),
          success
        );
        return Err(DrcomError::DataChallengeError(
          "Challenge data send error!".into(),
        ));
      }

      self.socket.recv_from(&mut self.buf).unwrap();

      if self.buf[0] == 0x02 {
        println!("Challenge success!");
        return Ok(());
      }

      if self.buf[0] == 0x07 {
        eprintln!("Challenge data error!");
        return Err(DrcomError::DataChallengeError(
          "Challenge data error!".into(),
        ));
      }

      eprintln!("Challenge failed! Retry...");
      self.try_times += 1;
    }

    eprintln!("Challenge failed! Try times: {}", self.try_times);
    Err(DrcomError::DataChallengeError("Challenge failed!".into()))
  }

  fn get_challenge_data(&self) -> Vec<u8> {
    vec![
      0x01,
      0x02 + self.try_times,
      rand::random::<u8>(),
      rand::random::<u8>(),
      0x09,
      0x00,
      0x00,
      0x00,
      0x00,
      0x00,
      0x00,
      0x00,
      0x00,
      0x00,
      0x00,
      0x00,
      0x00,
      0x00,
      0x00,
      0x00,
    ]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_challenge() {
    let mut conn = DrcomConnection::new("username", "password", 0);
    conn.challenge().unwrap();
  }
}
