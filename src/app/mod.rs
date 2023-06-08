pub mod modules;
pub mod utils;

use clap::Parser;

use crate::app::utils::cli;

use self::{
  modules::{
    connection::{self, DrcomConnection},
    generator::ChallengeGenerator,
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
        ConfigStore::get_instance()?.lock().unwrap().username = username.clone();
        ConfigStore::get_instance()?.lock().unwrap().password = password.clone();
        DrcomConnection::<ChallengeGenerator>::new()?.run()
      }
    }
  }
}
