use pnet::datalink;

pub struct Connection {
  pub username: String,
  pub password: String,
  pub mac: String,
  pub try_count: u8,
}

impl Connection {
  pub fn new(username: String, password: String) -> Self {
    let interfaces = datalink::interfaces();

    let mut mac = String::from("");
    for interface in interfaces {
      if interface.is_up() && interface.is_broadcast() && !interface.is_loopback() {
        let raw_mac = interface.mac.unwrap().to_string();
        mac = raw_mac.split(":").collect::<Vec<&str>>().join("");
        break;
      }
    }

    Self {
      username,
      password,
      mac,
      try_count: 0,
    }
  }

  fn get_challenge_data(&self) -> Vec<u8> {
    vec![
      0x01,
      0x02 + self.try_count,
      rand::random::<u8>(),
      rand::random::<u8>(),
      0x09,
    ]
  }

  pub fn loop_until(&self) {
    loop {
      println!("username: {}", self.username);
      println!("password: {}", self.password);
      println!("mac: {}", self.mac);

      std::thread::sleep(std::time::Duration::from_secs(5));
    }
  }
}
