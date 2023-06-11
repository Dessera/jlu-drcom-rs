use log::info;
use tokio::net::UdpSocket;

use crate::utils::{config::ConfigStore, crc, error::DrResult};

#[derive(Default)]
pub struct KeepAliveGenerator {
  keep_40_count: u8,
}

pub enum AliveType {
  FIRST,
  SECOND,
  EXTRA,
}

impl KeepAliveGenerator {
  pub fn get_keep_38(&self) -> DrResult<Vec<u8>> {
    let mut data = vec![0u8; 38];
    data[0] = 0xff;
    let (md5a, tail) = ConfigStore::get(|config| Ok((config.md5a, config.tail)))?;
    data[1..17].copy_from_slice(&md5a);
    data[20..36].copy_from_slice(&tail);
    data[36] = rand::random::<u8>();
    data[37] = rand::random::<u8>();
    Ok(data)
  }

  pub fn get_keep_40(&mut self, alive_type: AliveType) -> DrResult<Vec<u8>> {
    let mut data = vec![0u8; 40];
    let config = ConfigStore::get_instance()?;
    data[0] = 0x07;
    data[1] = self.keep_40_count;
    if self.keep_40_count == 0xff {
      self.keep_40_count = 0;
    } else {
      self.keep_40_count += 1;
    }
    data[2] = 0x20;
    data[3] = 0x00;
    data[4] = 0x0b;
    data[5] = match alive_type {
      AliveType::FIRST | AliveType::EXTRA => 0x01,
      AliveType::SECOND => 0x03,
    };
    if let AliveType::EXTRA = alive_type {
      data[6] = 0x0f;
      data[7] = 0x27;
    } else {
      let (aver1, aver2) = config.keep_alive_version;
      data[6] = aver1;
      data[7] = aver2;
    }
    data[8] = rand::random::<u8>();
    data[9] = rand::random::<u8>();
    data[16..20].copy_from_slice(&config.tail_2);
    if let AliveType::SECOND = alive_type {
      let tmp = crc(
        &data[0..24]
          .iter()
          .chain(&config.client_ip)
          .copied()
          .collect::<Vec<u8>>(),
      );
      data[24..28].copy_from_slice(&tmp);
      data[28..32].copy_from_slice(&config.client_ip);
    }
    Ok(data)
  }
}

impl KeepAliveGenerator {
  pub async fn keepalive(&mut self, socket: &mut UdpSocket) -> DrResult<()> {
    let mut buf = vec![0u8; 1024];
    // 38 pack first
    let keep_38 = self.get_keep_38()?;
    socket.send(&keep_38).await?;
    socket.recv(&mut buf).await?;
    ConfigStore::get_instance()?
      .keep_alive_version
      .clone_from(&(buf[28], buf[29]));

    // 40 extra
    if self.keep_40_count % 21 == 0 {
      let keep_40 = self.get_keep_40(AliveType::EXTRA)?;
      socket.send(&keep_40).await?;
      socket.recv(&mut buf).await?;
      info!("keepalive once round");
    }

    // 40 first
    let keep_40 = self.get_keep_40(AliveType::FIRST)?;
    socket.send(&keep_40).await?;
    socket.recv(&mut buf).await?;
    ConfigStore::get_instance()?
      .tail_2
      .copy_from_slice(&buf[16..20]);

    // 40 second
    let keep_40 = self.get_keep_40(AliveType::SECOND)?;
    socket.send(&keep_40).await?;
    socket.recv(&mut buf).await?;

    Ok(())
  }
}
