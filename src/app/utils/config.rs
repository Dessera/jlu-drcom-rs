use super::error::DrResult;
use crate::app::utils::error::DrcomError;
use lazy_static::lazy_static;
use log::error;
use std::sync::{Arc, Mutex, MutexGuard};

pub type ConfigResult<'a> = DrResult<MutexGuard<'a, ConfigStore>>;

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

  // pc or file config
  pub hostname: String,
  pub mac: [u8; 6],

  // static config
  pub primary_dns: [u8; 4],
  // dyn but do not need almost time
  pub secondary_dns: [u8; 4],
  // static config
  pub dhcp_server: [u8; 4],
}

impl ConfigStore {
  pub fn init() -> DrResult<()> {
    // init store immediately
    ConfigStore::get_instance()?;
    Ok(())
  }

  // implement in test case
  pub fn new() -> DrResult<Self> {
    // TODO: Find a way scan interfaces which suitable for windows
    //       I don't know why pnet conpile failed on mingw

    // TODO: Error handle, it's not a good idea to abort
    let hostnm: String = match hostname::get()?.into_string() {
      Ok(s) => s,
      Err(_) => {
        error!("cannot get hostname");
        return Err(DrcomError::OsError("cannot get hostname".to_string()));
      }
    };
    Ok(Self {
      salt: [0u8; 4],
      md5a: [0u8; 16],
      tail: [0u8; 16],
      tail_2: [0u8; 4],
      client_ip: [0u8; 4],
      keep_alive_version: (0, 0),
      username: String::new(),
      password: String::new(),
      hostname: hostnm,
      mac: [0xFC, 0x34, 0x97, 0x95, 0x27, 0xAF],
      primary_dns: [10, 10, 10, 10],

      // TODO: now the place use my dns server (consider remove this)
      secondary_dns: [202, 98, 18, 3],
      dhcp_server: [0u8; 4],
    })
  }

  // Singleton pattern for ConfigStore
  // pub fn get_instance() -> ConfigResult {
  //   static mut INSTANCE: Option<ConfigStore> = None;
  //   unsafe {
  //     if INSTANCE.is_none() {
  //       INSTANCE = Some(ConfigStore::new()?);
  //     }
  //     Ok(INSTANCE.clone())
  //   }
  // }
}

// Singleton pattern for ConfigStore
// Mutiple thread is not supported
lazy_static! {
  static ref INSTANCE: Mutex<ConfigStore> = {
    let config = ConfigStore::new().unwrap();
    Mutex::new(config)
  };
}

impl ConfigStore {
  pub fn get_instance() -> ConfigResult<'static> {
    match INSTANCE.lock() {
      Ok(config) => Ok(config),
      Err(_) => Err(DrcomError::LockError(
        "Failed to lock config instance".to_string(),
      )),
    }
  }
}
