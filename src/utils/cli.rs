use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

/// Subcommands
#[derive(Subcommand)]
pub enum Commands {
  Run {
    #[arg(help = "username of jlu mail", short, long)]
    username: String,
    #[arg(help = "password of jlu mail", short, long)]
    password: String,
    #[arg(
      help = "mac address of your device (eg: 12:34:56:78:9A:BC)",
      short,
      long
    )]
    mac: String,
    #[arg(help = "receive timeout in seconds", short, long, default_value = "5")]
    timeout: u64,
  },
  Clear {}
}
