use clap::Parser;

use crate::utils::cli;

use crate::{
  modules::{
    connection::DrcomConnection,
    generator::{ChallengeGenerator, KeepAliveGenerator, LoginGenerator},
  },
  utils::{config::ConfigStore, error::DrResult},
};

pub struct App {
  pub cli_args: cli::Cli,
}

impl App {
  pub fn new() -> Self {
    Self {
      cli_args: cli::Cli::parse(),
    }
  }

  pub fn run(&self) -> DrResult<()> {
    match &self.cli_args.command {
      cli::Commands::Run {
        username,
        password,
        mac,
      } => {
        {
          let mut config = ConfigStore::get_instance()?;
          config.username = username.clone();
          config.password = password.clone();
          // parse mac address 12:34:56:78:9A:BC to [u8; 6]
          config.mac = mac
            .split(':')
            .map(|x| u8::from_str_radix(x, 16))
            .collect::<Result<Vec<u8>, _>>()?
            .try_into()
            .unwrap_or([0; 6]);
        }
        DrcomConnection::<ChallengeGenerator, LoginGenerator, KeepAliveGenerator>::new()?.run()
      }
    }
  }
}
