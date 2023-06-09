use clap::Parser;
use jlu_drcom_rs::{
  app::app_run,
  utils::{config::ConfigStore, error::DrcomError, log_file::get_logfile},
};
use log::{error, info, LevelFilter};
use simplelog::WriteLogger;

#[tokio::main]
async fn main() -> Result<(), DrcomError> {
  // 初始化Cli
  // 在这之后初始化会导致先输出日志
  let cli_args = jlu_drcom_rs::utils::cli::Cli::parse();

  // 初始化Logger
  // 如果失败了，只能使用eprintln!输出
  let fd = get_logfile().unwrap_or_else(|e| {
    eprintln!("Create log file: {}", e);
    std::process::exit(1);
  });
  WriteLogger::init(LevelFilter::Trace, simplelog::Config::default(), fd).unwrap_or_else(|e| {
    eprintln!("Logger init: {}", e);
    std::process::exit(1);
  });
  info!("Logger init success.");

  // 初始化设置
  // 会生成单例ConfigStore，并初始化一些值
  ConfigStore::init().unwrap_or_else(|e| {
    error!("{}", e);
    std::process::exit(1);
  });
  info!("Config init success.");

  // 运行主程序
  // 使用函数初始化，个人感觉没必要构造类
  info!("App start.");
  app_run(cli_args).await.unwrap_or_else(|e| {
    error!("{}", e);
    std::process::exit(1);
  });
  info!("App exit.");
  Ok(())
}
