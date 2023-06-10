use log::info;

use crate::utils::error::DrResult;
use tokio::signal::unix::{signal, SignalKind};

use super::generator::{ChallengeGenerator, KeepAliveGenerator, LoginGenerator};

pub struct DrcomConnection {
  pub socket: tokio::net::UdpSocket,
  pub challenger: ChallengeGenerator,
  pub loginer: LoginGenerator,
  pub aliver: KeepAliveGenerator,
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
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(20));
    loop {
      tokio::select! {
        _ = self.signal_task() => {
          break;
        },
        _ = interval.tick() => {
          self.aliver.keepalive(&mut self.socket).await?;
        }
      }
    }

    info!("receive signal, start logout");
    self.loginer.logout(&mut self.socket).await?;
    info!("logout success");

    Ok(())
  }

  async fn signal_task(&self) {
    let mut sigint = signal(SignalKind::interrupt()).unwrap();
    let mut sigterm = signal(SignalKind::terminate()).unwrap();
    let mut sigquit = signal(SignalKind::quit()).unwrap();
    tokio::select! {
      _ = sigint.recv() => {
        info!("receive SIGINT");
      },
      _ = sigterm.recv() => {
        info!("receive SIGTERM");
      },
      _ = sigquit.recv() => {
        info!("receive SIGQUIT");
      }
    }
  }
}
