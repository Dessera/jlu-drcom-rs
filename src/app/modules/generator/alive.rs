use crate::app::utils::error::DrResult;
use crate::app::utils::error::DrcomError;
use crate::app::utils::interface::Ichallenge;
use crate::app::utils::interface::Ikeepalive;
use log::{error, info};

pub struct KeepAliveGenerator {}

pub struct KeepAlive38Generator {}

pub struct KeepAlive40_1Generator {}

pub struct KeepAlive40_2Generator {}

pub struct KeepAlive40_exGenerator {}

impl Ikeepalive for KeepAlive38Generator {
  fn get_keepalive_data(&self) -> DrResult<Vec<u8>> {
    todo!()
  }
  fn keepalive(&mut self) -> DrResult<()> {
    todo!()
  }
}

impl Ikeepalive for KeepAlive40_1Generator {
  fn get_keepalive_data(&self) -> DrResult<Vec<u8>> {
    todo!()
  }
  fn keepalive(&mut self) -> DrResult<()> {
    todo!()
  }
}

impl Ikeepalive for KeepAlive40_2Generator {
  fn get_keepalive_data(&self) -> DrResult<Vec<u8>> {
    todo!()
  }
  fn keepalive(&mut self) -> DrResult<()> {
    todo!()
  }
}

impl Ikeepalive for KeepAlive40_exGenerator {
  fn get_keepalive_data(&self) -> DrResult<Vec<u8>> {
    todo!()
  }
  fn keepalive(&mut self) -> DrResult<()> {
    todo!()
  }
}

impl Ikeepalive for KeepAliveGenerator {
  fn get_keepalive_data(&self) -> DrResult<Vec<u8>> {
    todo!()
  }
  fn keepalive(&mut self) -> DrResult<()> {
    todo!()
  }
}
