use std::time::Duration;

use log::info;

use crate::utils::{error::DrResult, sock::DrSocket};

use super::generator::{ChallengeGenerator, KeepAliveGenerator, LoginGenerator};

pub struct DrcomConnection {
  pub socket: DrSocket,
  pub challenger: ChallengeGenerator,
  pub loginer: LoginGenerator,
  pub aliver: KeepAliveGenerator,
}

impl DrcomConnection {
  pub async fn create(timeout: u64) -> DrResult<Self> {
    let socket = DrSocket::create("0.0.0.0:0", Duration::from_secs(timeout)).await?;
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
        res = self.signal_task() => {
          res?;
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

  #[cfg(target_os = "linux")]
  async fn signal_task(&self) -> DrResult<()> {
    use tokio::signal::unix::{signal, SignalKind};
    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;
    let mut sigquit = signal(SignalKind::quit())?;
    tokio::select! {
      _ = sigint.recv() => {},
      _ = sigterm.recv() => {},
      _ = sigquit.recv() => {}
    }
    Ok(())
  }

  #[cfg(target_os = "windows")]
  async fn signal_task(&self) -> DrResult<()> {
    use tokio::signal::windows::{ctrl_break, ctrl_c};
    let mut sigint = ctrl_c()?;
    let mut sigterm = ctrl_break()?;
    tokio::select! {
      _ = sigint.recv() => {},
      _ = sigterm.recv() => {}
    }
    Ok(())
  }
}
