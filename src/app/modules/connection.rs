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
  pub mac_addr: Vec<u8>,
  pub destination: String,
  socket: UdpSocket,
  try_times: u8,
  clg_buf: [u8; 1024],
}

impl DrcomConnection {
  pub fn new(username: &str, password: &str, mac: Vec<u8>) -> Self {
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

  /**
   * Generate login data.
   *
   * Data format:
   *      0x03,0x01,0x00    Login data header
   *      20 + usr_len      Username length
   *      md5_hash          MD5 hash (0x03 + 0x01 + challenge_data[4..8] + password)
   *      username          Username (length: 36)
   *      0x20              Control Check status
   *      0x03              Adapter num
   *      mac_addr          md5_hash as u8 array ^ mac_addr
   *      md5_hash          MD5 hash (0x01 + password + salt + 0x00 * 4)
   *      ip address        0x01 ip_address
   *      ip 2              ip_address
   *      ip 3              ip_address
   *      ip 4              ip_address
   *      md5_hash          MD5 hash (data + 0x14 + 0x00 + 0x07 + 0x0b) 8 bytes
   *      0x01              ipdog
   *      0x00 * 4          delimeter
   *      host_name         32 bytes
   *      primary_dns       primary_dns
   *      dhcp_server       dhcp_server
   *      second_dns        second_dns
   *      0x00 * 8          delimeter
   *      0x94 0x00 * 3     unknown
   *      0x06 0x00 * 3       os major
   *      0x02 0x00 * 3       os minor
   *      0xf0 0x23 0x00 * 2  os build
   *      0x02 0x00 * 3       os unknown
   *      0x44 0x72 0x43 0x4f 0x4d 0x00 0xcf 0x07 0x68
   *      0x00 * 55
   *      0x33 0x64 0x63 0x37 0x39 0x66 0x35 0x32 0x31 0x32 0x65 0x38 0x31 0x37 0x30 0x61 0x63 0x66 0x61 0x39 0x65 0x63 0x39 0x35 0x66 0x31 0x64 0x37 0x34 0x39 0x31 0x36 0x35 0x34 0x32 0x62 0x65 0x37 0x62 0x31
   *      0x00 * 24
   *      0x68 0x00         Auth version
   *      0x00 pwd_len
   *      md5_hash          ror(0x03 0x01 salt pwd, pwd)
   *      0x02 0x0c
   *      data 0x01 0x26 0x07 0x11 0x00 0x00 mac
   *      0x00 0x00
   *      mac
   *      0x00 * pwd_len % 4
   *      0x60 0xa2         unknown
   *      0x00 * 28
   *      
   */
  fn get_login_data(&self) -> Vec<u8> {
    let mut data = vec![0x03, 0x01, 0x00, self.username.len() as u8 + 20];

    let salt = &self.clg_buf[4..8];
    let md5_hash = md5::compute(
      [
        vec![0x03, 0x01],
        salt.to_vec(),
        self.password.as_bytes().to_vec(),
      ]
      .concat(),
    );
    data.extend_from_slice(&md5_hash.as_slice());

    let mut usr_data = self.username.as_bytes().to_vec();
    usr_data.resize(36, 0);
    data.extend_from_slice(&usr_data);

    data.extend_from_slice(&[0x20, 0x03]);

    // md5 ^ mac

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
    let mut conn = DrcomConnection::new("username", "password", Vec::new());
    conn.challenge().unwrap();
  }

  #[test]
  fn test_get_login_data() {
    let mut conn = DrcomConnection::new("username", "zxcvb123456", Vec::new());
    conn.challenge().unwrap();
    let data = conn.get_login_data();
    println!("{:?}", data);
  }
}
