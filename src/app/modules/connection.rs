use std::net::UdpSocket;

use crate::app::utils::{
  error::{DrResult, DrcomError},
  interface::{Ichallenge, Ilogin},
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
  clg_buf: [u8; 1024],
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
      clg_buf: [0; 1024],
    }
  }

  pub fn clear(&mut self) {
    self.try_times = 0;
    self.clg_buf = [0; 1024];
  }
}

impl Ichallenge for DrcomConnection {
  fn challenge(&mut self) -> DrResult<()> {
    self.clear();
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

      self.socket.recv_from(&mut self.clg_buf).unwrap();

      if self.clg_buf[0] == 0x02 {
        println!("Challenge success!");
        return Ok(());
      }

      if self.clg_buf[0] == 0x07 {
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

  /**
   * Generate challenge data.
   *
   * Data format:
   *      0x01              Challenge data header
   *      0x02 + try_times  Challenge data try times
   *      rand()            Random number
   *      rand()            Random number
   *      0x09              Challenge data tail
   *      0x00              Fill 0 to maintain the length of 20 bytes
   */
  fn get_challenge_data(&self) -> Vec<u8> {
    let mut data = vec![
      0x01,
      0x02 + self.try_times,
      rand::random::<u8>(),
      rand::random::<u8>(),
      0x09,
    ];
    data.resize(20, 0);
    data
  }
}

impl Ilogin for DrcomConnection {
  fn login(&mut self) -> DrResult<()> {
    todo!()
  }

  fn logout(&mut self) -> DrResult<()> {
    todo!()
  }

  fn get_login_data(&self) -> Vec<u8> {
    let mut code_array = vec![0x03, 0x01];

    // salt
    code_array.extend_from_slice(&self.clg_buf[4..8]);

    // password
    code_array.extend_from_slice(&self.password.as_bytes());

    // md5 hash
    let md5_hash = format!("{:x}", md5::compute(code_array));

    let mut data = vec![0x03, 0x01, 0x00, self.username.len() as u8 + 20];
    data.resize(338, 0);
    data
  }

  fn get_logout_data(&self) -> Vec<u8> {
    todo!()
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
