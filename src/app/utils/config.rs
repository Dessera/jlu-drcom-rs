use std::{
  ops::Deref,
  sync::{Arc, Mutex},
};

pub struct ConfigStore {
  // read runtime
  pub salt: [u8; 4],
  pub tail: [u8; 16],

  // read from command line
  pub username: String,
  pub password: String,

  // read from pc or config file
  pub hostname: String,
  pub mac: [u8; 6],
  pub primary_dns: [u8; 4],
  pub secondary_dns: [u8; 4],
  pub dhcp_server: [u8; 4],
  pub client_ip: [u8; 4],
  // in theory, clinet_ip has 4 positions, but only the first one is useful for me now
}

// global config store

impl ConfigStore {
  pub fn init() {
    // init store immediately
    ConfigStore::get_instance();
  }

  // implement in test case
  pub fn new() -> Self {
    Self {
      salt: [0u8; 4],
      tail: [0u8; 16],
      username: String::new(),
      password: String::new(),
      hostname: String::new(),
      mac: [0u8; 6],
      primary_dns: [0u8; 4],
      secondary_dns: [0u8; 4],
      dhcp_server: [0u8; 4],
      client_ip: [0u8; 4],
    }
  }

  // Singleton pattern for ConfigStore
  pub fn get_instance() -> Arc<Mutex<ConfigStore>> {
    static mut INSTANCE: Option<Arc<Mutex<ConfigStore>>> = None;
    unsafe {
      INSTANCE
        .get_or_insert_with(|| Arc::new(Mutex::new(ConfigStore::new())))
        .clone()
    }
  }
}
