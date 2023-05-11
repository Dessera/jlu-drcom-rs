use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  #[command(about = "run the application")]
  Run{
    #[arg(short, long, help = "jlu.edu.cn username")]
    username: Option<String>,

    #[arg(short, long, help = "jlu.edu.cn password")]
    password: Option<String>,
  }
}
