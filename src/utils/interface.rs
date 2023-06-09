use super::error::DrResult;

pub trait Ichallenge {
  fn challenge(&mut self, socket: &mut std::net::UdpSocket) -> DrResult<()>;
  fn get_challenge_data(&self) -> DrResult<Vec<u8>>;
}

pub trait Ilogin {
  fn login(&mut self, socket: &mut std::net::UdpSocket) -> DrResult<()>;
  fn get_login_data(&self) -> DrResult<Vec<u8>>;
  fn logout(&mut self, socket: &mut std::net::UdpSocket) -> DrResult<()>;
  fn get_logout_data(&self) -> DrResult<Vec<u8>>;
}

pub trait Ikeepalive {
  fn keepalive(&mut self, socket: &mut std::net::UdpSocket) -> DrResult<()>;
  fn get_keepalive_data(&self) -> DrResult<Vec<u8>>;
}
