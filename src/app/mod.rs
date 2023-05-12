pub mod modules;
pub mod utils;

use clap::Parser;

use crate::app::utils::cli;

use self::utils::interface::Ichallenge;

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
      cli::Commands::Run {
        username,
        password,
        mac,
      } => {
        let mac_addr = mac
          .split(':')
          .fold(0, |acc, x| acc * 256 + u64::from_str_radix(x, 16).unwrap());
        let mut connection =
          modules::connection::DrcomConnection::new(username, password, mac_addr);
        connection.challenge().unwrap();
      }
    }
  }
}
