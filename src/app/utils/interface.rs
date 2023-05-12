use super::error::DrResult;

pub trait Ichallenge {
  fn challenge(& mut self) -> DrResult<()>;
  fn get_challenge_data(&self) -> Vec<u8>;
}

pub trait Ilogin {
  fn login(& mut self) -> DrResult<()>;
  fn get_login_data(&self) -> Vec<u8>;
  fn logout(& mut self) -> DrResult<()>;
  fn get_logout_data(&self) -> Vec<u8>;
}

pub trait Ikeepalive {
  fn keepalive(& mut self) -> DrResult<()>;
  fn get_keepalive_data(&self) -> Vec<u8>;
}
