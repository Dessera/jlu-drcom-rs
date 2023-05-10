use pnet::datalink;

pub struct Connection {
  pub username: String,
  pub password: String,
  pub mac: String,
}

impl Connection {
  pub fn new(username: String, password: String) -> Self {
    let interfaces = datalink::interfaces();

    let mut mac = String::from("");
    for interface in interfaces {
      if interface.is_up() && interface.is_broadcast() && !interface.is_loopback() {
        mac = interface.mac.unwrap().to_string();
        break;
      }
    }

    Self {
      username,
      password,
      mac
    }
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
