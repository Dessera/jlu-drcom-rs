use jlu_drcom_rs::{app::App, utils::config::ConfigStore};
use log::{error, info};

fn main() {
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
  App::new().run().unwrap_or_else(|e| {
    error!("{}", e);
    std::process::exit(1);
  })
}
