pub mod cli;
pub mod config;
pub mod link;

use clap::Parser;

pub struct App {
  pub cli_args: cli::Cli,
  pub setting: config::Configuration,
}

impl App {
  pub fn new() -> Self {
    let cli_args = cli::Cli::parse();
    Self {
      cli_args,
      setting: config::Configuration::new(),
    }
  }

  pub fn run(&self) {
    match &self.cli_args.command {
      cli::Commands::Run { username, password } => {
        link::Connection::new(username.clone().unwrap(), password.clone().unwrap()).loop_until();
      }
    }
  }
}
