pub mod modules;
pub mod utils;

use clap::Parser;

use crate::app::utils::cli;

pub struct App {
  pub cli_args: cli::Cli,
}

impl App {
  pub fn new() -> Self {
    Self {
      cli_args: cli::Cli::parse(),
    }
  }

  pub fn run(&self) {
    match &self.cli_args.command {
      cli::Commands::Run { username, password } => {}
    }
  }
}
