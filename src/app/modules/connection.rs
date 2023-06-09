use log::info;

use crate::app::utils::{
  error::DrResult,
  interface::{Ichallenge, Ikeepalive, Ilogin},
};

pub struct DrcomConnection<Challenger, Loginer, Aliver>
where
  Challenger: Ichallenge + Default,
  Loginer: Ilogin + Default,
  Aliver: Ikeepalive + Default,
{
  pub socket: std::net::UdpSocket,
  pub challenger: Challenger,
  pub loginer: Loginer,
  pub aliver: Aliver,

  pub running: bool,
}

impl<Challenger, Loginer, Aliver> DrcomConnection<Challenger, Loginer, Aliver>
where
  Challenger: Ichallenge + Default,
  Loginer: Ilogin + Default,
  Aliver: Ikeepalive + Default,
{
  pub fn new() -> DrResult<Self> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("10.100.61.3:61440")?;
    socket.set_read_timeout(Some(std::time::Duration::from_secs(5)))?;
    Ok(Self {
      socket,
      challenger: Challenger::default(),
      loginer: Loginer::default(),
      aliver: Aliver::default(),

      running: true,
    })
  }

  pub fn run(&mut self) -> DrResult<()> {
    info!("start data challenge");
    self.challenger.challenge(&mut self.socket)?;
    info!("data challenge success");

    info!("start login");
    self.loginer.login(&mut self.socket)?;
    info!("login success");

    info!("start keep alive");
    while self.running {
      self.aliver.keepalive(&mut self.socket)?;
      std::thread::sleep(std::time::Duration::from_secs(20));
    }

    Ok(())
  }
}
