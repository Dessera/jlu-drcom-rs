use rand::random;

use crate::utils::{
  checksum,
  config::ConfigStore,
  error::{DrResult, DrcomError},
  ror, sock::DrSocket,
};

#[derive(Default)]
pub struct LoginGenerator {}

impl LoginGenerator {
  fn get_login_data(&self) -> DrResult<Vec<u8>> {
    // config instance get
    let mut config = ConfigStore::get_instance()?;
    let mut data = vec![0u8; 349];
    let password_len = if config.password.len() > 16 {
      16
    } else {
      config.password.len() as usize
    };
    // code:0x03 type:0x01 EOF:0x00
    data[0] = 0x03;
    data[1] = 0x01;
    data[2] = 0x00;
    data[3] = config.username.len() as u8 + 20;

    // md5a: code type salt password
    let md5a = md5::compute(
      [0x03, 0x01]
        .iter()
        .chain(config.salt.iter())
        .chain(config.password.as_bytes())
        .copied()
        .collect::<Vec<u8>>(),
    );
    data[4..20].copy_from_slice(&md5a.0);
    config.md5a = md5a.0;

    // username 36 bytes (fill with 0x00 if less than 36)
    let mut usrname_data = config.username.as_bytes().to_vec();
    usrname_data.resize(36, 0x00);
    data[20..56].copy_from_slice(&usrname_data);

    // control: 0x20 adapter: 0x05
    data[56] = 0x20;
    data[57] = 0x05;

    // md5a 0..6 xor mac 0..6
    for i in 0..6 {
      data[58 + i] = md5a.0[i] ^ config.mac[i];
    }

    // md5b: 0x01 password salt 0x00 0x00 0x00 0x00
    let md5b = md5::compute(
      [0x01]
        .iter()
        .chain(config.password.as_bytes())
        .chain(config.salt.iter())
        .chain([0x00, 0x00, 0x00, 0x00].iter())
        .copied()
        .collect::<Vec<u8>>(),
    );
    data[64..80].copy_from_slice(&md5b.0);

    // ip number
    data[80] = 0x01;
    // ip address
    data[81..85].copy_from_slice(&config.client_ip);
    data[85..97].copy_from_slice(&[0x00; 12]);

    // md5c[0..8] current data 0x14 0x00 0x07 0x0b
    let md5c = md5::compute(
      [].iter()
        .chain(&data[0..97])
        .chain([0x14, 0x00, 0x07, 0x0b].iter())
        .copied()
        .collect::<Vec<u8>>(),
    );
    data[97..105].copy_from_slice(&md5c.0[0..8]);

    // ipdog:0x01
    data[105] = 0x01;

    // 0x00 0x00 0x00 0x00
    // hostname 32 bytes (fill with 0x00 if less than 32)
    let mut hostname_data = config.hostname.as_bytes().to_vec();
    hostname_data.resize(32, 0x00);
    data[110..142].copy_from_slice(&hostname_data);
    // primary dns
    data[142..146].copy_from_slice(&config.primary_dns);
    // dhcp server
    data[146..150].copy_from_slice(&config.dhcp_server);
    // secondary dns
    data[150..154].copy_from_slice(&config.secondary_dns);

    // delimiter to 162
    data[162] = 0x94; //unknown 162+4=166
    data[166] = 0x06; //os major 166+4=170
    data[170] = 0x02; //os minor 170+4=174
    data[174] = 0xf0; //os build
    data[175] = 0x23; //os build 174+4=178
    data[178] = 0x02; //os unknown 178+4=182

    //DRCOM CHECK
    data[182] = 0x44;
    data[183] = 0x72;
    data[184] = 0x43;
    data[185] = 0x4f;
    data[186] = 0x4d;
    data[187] = 0x00;
    data[188] = 0xcf;
    data[189] = 0x07;
    data[190] = 0x6a;

    // 0x00 to 246
    data[246..286].copy_from_slice("1c210c99585fd22ad03d35c956911aeec1eb449b".as_bytes());
    // 0x00 to 310
    data[310] = 0x6a;
    data[313] = password_len as u8;
    // ror md5a x password (password_len)
    let ror_data = ror(&md5a.0, config.password.as_bytes());
    // 314..password_len+314
    data[314..password_len + 314].copy_from_slice(&ror_data[0..password_len]);
    data[password_len + 314] = 0x02;
    data[password_len + 315] = 0x0c;
    // checksum: 0x01 0x26 0x07 0x11 0x00 0x00 mac
    let checksum_val = checksum(
      &[0x01, 0x26, 0x07, 0x11, 0x00, 0x00]
        .iter()
        .chain(config.mac.iter())
        .copied()
        .collect::<Vec<u8>>(),
    );
    data[password_len + 316..password_len + 320].copy_from_slice(&checksum_val);
    data[password_len + 320] = 0x00;
    data[password_len + 321] = 0x00;

    // mac
    data[password_len + 322..password_len + 328].copy_from_slice(&config.mac);

    let zero_count = (4 - password_len % 4) % 4;
    for i in 0..zero_count {
      data[password_len + 328 + i] = 0x00;
    }
    // random 2 bytes
    data[password_len + 328 + zero_count] = random();
    data[password_len + 329 + zero_count] = random();

    let new_len = 334 + (password_len - 1) / 4 * 4;
    data.resize(new_len, 0x00);
    Ok(data)
  }

  fn get_logout_data(&self) -> DrResult<Vec<u8>> {
    let config = ConfigStore::get_instance()?;
    let mut data = vec![0u8; 80];
    data[0] = 0x06; //code
    data[1] = 0x01; //type
    data[2] = 0x00; //EOF
    data[3] = config.username.len() as u8 + 20;

    // md5: 0x06 0x01 salt password
    let md5a = md5::compute(
      [0x06, 0x01]
        .iter()
        .chain(&config.salt)
        .chain(config.password.as_bytes())
        .copied()
        .collect::<Vec<u8>>(),
    );
    data[4..20].copy_from_slice(&md5a.0);

    // uname ljust 36
    let mut uname = config.username.as_bytes().to_vec();
    uname.resize(36, 0x00);
    data[20..56].copy_from_slice(&uname);

    // 0x20 0x05
    data[56] = 0x20;
    data[57] = 0x05;

    // mac xor md5[0..6]
    let mut mac_xor = [0u8; 6];
    for i in 0..6 {
      mac_xor[i] = config.mac[i] ^ md5a.0[i];
    }
    data[58..64].copy_from_slice(&mac_xor);

    // tail
    data[64..80].copy_from_slice(&config.tail);
    Ok(data)
  }

  pub async fn login(&mut self, socket: &mut DrSocket) -> DrResult<()> {
    let data = self.get_login_data()?;
    socket.send(&data).await?;
    let mut buf = [0; 1024];
    socket.recv_with_timeout(&mut buf).await?;
    if buf[0] == 0x04 {
      // save tail
      ConfigStore::get_instance()?
        .tail
        .copy_from_slice(&buf[23..39]);
      return Ok(());
    }
    if buf[0] == 0x05 && buf[4] == 0x0b {
      return Err(DrcomError::LoginError("Invalid MAC address".to_string()));
    } else if buf[0] == 0x05 {
      return Err(DrcomError::LoginError(
        "Invalid username or password".to_string(),
      ));
    } else {
      return Err(DrcomError::LoginError("Unknown error".to_string()));
    }
  }

  pub async fn logout(&mut self, socket: &mut DrSocket) -> DrResult<()> {
    let data = self.get_logout_data()?;
    socket.send(&data).await?;
    let mut buf = [0; 512];
    socket.recv_with_timeout(&mut buf).await?;
    if buf[0] == 0x04 {
      return Ok(());
    } else {
      return Err(DrcomError::LogoutError("Unknown error".to_string()));
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::modules::generator::ChallengeGenerator;

  #[tokio::test]
  #[ignore = "need a valid config"]
  async fn test_login() {
    simple_logger::init().unwrap();
    ConfigStore::init().unwrap();

    // test settings
    ConfigStore::get_instance().unwrap().username = "username".to_string();
    ConfigStore::get_instance().unwrap().password = "password".to_string();

    let mut cgen = ChallengeGenerator::default();
    let mut lgen = LoginGenerator::default();
    let mut socket = tokio::net::UdpSocket::bind("0.0.0.0:0").await.unwrap();
    socket.connect("10.100.61.3:61440").await.unwrap();
    let mut socket = DrSocket::new(socket);
    let clg_res = cgen.challenge(&mut socket).await;
    assert!(clg_res.is_ok());
    let login_res = lgen.login(&mut socket).await;
    assert!(login_res.is_ok());
  }
}
