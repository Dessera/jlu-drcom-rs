use jlu_drcom_rs::{
  app::run,
  utils::{config::ConfigStore, error::DrcomError},
};
use log::{error, info};

#[tokio::main]
async fn main() -> Result<(), DrcomError> {
  // init logger
  simple_logger::init().unwrap_or_else(|_| {
    eprintln!("Logger init failed.");
    std::process::exit(1);
  });
  info!("Logger init success.");

  // init config
  ConfigStore::init().unwrap_or_else(|e| {
    error!("{}", e);
    std::process::exit(1);
  });
  info!("Config init success.");

  // run app
  info!("App start.");
  run().await.unwrap_or_else(|e| {
    error!("{}", e);
    std::process::exit(1);
  });
  info!("App exit.");
  Ok(())
}
