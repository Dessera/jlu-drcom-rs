use log::info;

use crate::utils::error::DrResult;

use super::generator::{ChallengeGenerator, KeepAliveGenerator, LoginGenerator};

pub struct DrcomConnection {
  pub socket: tokio::net::UdpSocket,
  pub challenger: ChallengeGenerator,
  pub loginer: LoginGenerator,
  pub aliver: KeepAliveGenerator,

  pub running: bool,
}

impl DrcomConnection {
  pub async fn create() -> DrResult<Self> {
    let socket = tokio::net::UdpSocket::bind("0.0.0.0:0").await?;
    socket.connect("10.100.61.3:61440").await?;
    Ok(Self {
      socket,
      challenger: ChallengeGenerator::default(),
      loginer: LoginGenerator::default(),
      aliver: KeepAliveGenerator::default(),

      running: true,
    })
  }

  pub async fn run(&mut self) -> DrResult<()> {
    info!("start data challenge");
    self.challenger.challenge(&mut self.socket).await?;
    info!("data challenge success");

    info!("start login");
    self.loginer.login(&mut self.socket).await?;
    info!("login success");

    info!("start keep alive");
    while self.running {
      self.aliver.keepalive(&mut self.socket).await?;
      std::thread::sleep(std::time::Duration::from_secs(20));
    }

    Ok(())
  }
}
