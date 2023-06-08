use super::error::DrResult;
use crate::app::utils::error::DrcomError;
use log::error;
use std::sync::{Arc, Mutex};

pub type ConfigResult = DrResult<Arc<Mutex<ConfigStore>>>;

pub struct ConfigStore {
  // runtime config
  pub salt: [u8; 4],
  pub tail: [u8; 16],
  pub client_ip: [u8; 4],

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
      tail: [0u8; 16],
      client_ip: [0u8; 4],
      username: String::new(),
      password: String::new(),
      hostname: hostnm,
      mac: [0u8; 6],
      primary_dns: [10, 10, 10, 10],

      // TODO: now the place use my dns server (consider remove this)
      secondary_dns: [202, 98, 18, 3],
      dhcp_server: [0u8; 4],
    })
  }

  // Singleton pattern for ConfigStore
  pub fn get_instance() -> ConfigResult {
    static mut INSTANCE: Option<Arc<Mutex<ConfigStore>>> = None;
    unsafe {
      Ok(match INSTANCE {
        Some(ref ins) => ins.clone(),
        None => {
          let ins = Arc::new(Mutex::new(ConfigStore::new()?));
          INSTANCE = Some(ins.clone());
          ins
        }
      })
    }
  }
}
