pub mod modules;
pub mod utils;

use clap::Parser;

use crate::app::utils::cli;

use self::{
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
      cli::Commands::Run { username, password } => {
        ConfigStore::get_instance()?.username = username.clone();
        ConfigStore::get_instance()?.password = password.clone();
        DrcomConnection::<ChallengeGenerator, LoginGenerator, KeepAliveGenerator>::new()?.run()
      }
    }
  }
}
