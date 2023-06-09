use log::info;

use crate::app::utils::{
  error::DrResult,
  interface::{Ichallenge, Ilogin},
};

pub struct DrcomConnection<Challenger, Loginer>
where
  Challenger: Ichallenge,
  Loginer: Ilogin,
{
  pub socket: std::net::UdpSocket,
  pub challenger: Challenger,
  pub loginer: Loginer,
}

impl<Challenger, Loginer> DrcomConnection<Challenger, Loginer>
where
  Challenger: Ichallenge + Default,
  Loginer: Ilogin + Default,
{
  pub fn new() -> DrResult<Self> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("10.100.61.3:61440")?;
    socket.set_read_timeout(Some(std::time::Duration::from_secs(5)))?;
    Ok(Self {
      socket,
      challenger: Challenger::default(),
      loginer: Loginer::default(),
    })
  }

  pub fn run(&mut self) -> DrResult<()> {
    info!("start data challenge");
    self.challenger.challenge(&mut self.socket)?;
    info!("data challenge success");

    info!("start login");
    self.loginer.login(&mut self.socket)?;
    info!("login success");

    Ok(())
  }
}
