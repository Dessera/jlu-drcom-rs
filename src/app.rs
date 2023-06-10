use clap::Parser;

use crate::utils::cli;

use crate::{
  modules::connection::DrcomConnection,
  utils::{config::ConfigStore, error::DrResult},
};

pub async fn run() -> DrResult<()> {
  let cli_args = cli::Cli::parse();
  match cli_args.command {
    cli::Commands::Run {
      username,
      password,
      mac,
    } => {
      ConfigStore::modify(|config| {
        config.username = username.clone();
        config.password = password.clone();
        // parse mac address 12:34:56:78:9A:BC to [u8; 6]
        config.mac = mac
          .split(':')
          .map(|x| u8::from_str_radix(x, 16))
          .collect::<Result<Vec<u8>, _>>()?
          .try_into()
          .unwrap_or([0; 6]);
        Ok(())
      })?;
      DrcomConnection::create().await?.run().await
    }
  }
}
