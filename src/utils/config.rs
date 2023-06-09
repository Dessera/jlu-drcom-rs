use super::error::DrResult;
use crate::utils::error::DrcomError;
use lazy_static::lazy_static;
use log::error;
use std::sync::{Mutex, MutexGuard};

/// ConfigResult is a type alias of DrResult<MutexGuard<'a, ConfigStore>>.
pub type ConfigResult<'a> = DrResult<MutexGuard<'a, ConfigStore>>;

/// ConfigStore is a singleton struct that stores all the config
/// of the program.
/// Basically, it contains 4 parts:
/// 1. runtime config, which is generated during runtime
/// 2. command line args, which is parsed from command line
/// 3. pc or file config, which is read from pc or file
/// 4. static config, which is hard coded in the program
/// 
/// Note that the config is not thread safe, and it is not necessary to
/// make it thread safe.
pub struct ConfigStore {
  // runtime config
  pub salt: [u8; 4],
  pub md5a: [u8; 16],
  pub tail: [u8; 16],
  pub tail_2: [u8; 4],
  pub client_ip: [u8; 4],
  pub keep_alive_version: (u8, u8),

  // command line args
  pub username: String,
  pub password: String,
  pub mac: [u8; 6],

  // pc or file config
  pub hostname: String,

  // static config
  pub primary_dns: [u8; 4],
  // dyn but do not need almost time
  pub secondary_dns: [u8; 4],
  // static config
  pub dhcp_server: [u8; 4],
}

impl ConfigStore {
  /// Init the config store.
  /// It will read the hostname from the system.
  /// TODO: read the mac address from the system.
  /// TODO: save / read the config from the file.
  pub fn init() -> DrResult<()> {
    let mut config = ConfigStore::get_instance()?;
    config.hostname = match hostname::get()?.into_string() {
      Ok(s) => s,
      Err(_) => {
        error!("cannot get hostname");
        return Err(DrcomError::OsError("cannot get hostname".to_string()));
      }
    };
    Ok(())
  }

  /// Create a new ConfigStore.
  pub fn new() -> Self {
    Self {
      salt: [0u8; 4],
      md5a: [0u8; 16],
      tail: [0u8; 16],
      tail_2: [0u8; 4],
      client_ip: [0u8; 4],
      keep_alive_version: (0, 0),
      username: String::new(),
      password: String::new(),
      hostname: String::new(),
      mac: [0xFC, 0x34, 0x97, 0x95, 0x27, 0xAF],
      primary_dns: [10, 10, 10, 10],

      secondary_dns: [0u8; 4],
      dhcp_server: [0u8; 4],
    }
  }
}

// Singleton pattern for ConfigStore
// Mutiple thread is not supported
lazy_static! {
  static ref INSTANCE: Mutex<ConfigStore> = {
    let config = ConfigStore::new();
    Mutex::new(config)
  };
}

impl ConfigStore {
  /// Get the instance of ConfigStore.
  /// It will return a ConfigResult.
  pub fn get_instance() -> ConfigResult<'static> {
    match INSTANCE.lock() {
      Ok(config) => Ok(config),
      Err(_) => Err(DrcomError::LockError(
        "Failed to lock config instance".to_string(),
      )),
    }
  }
}
