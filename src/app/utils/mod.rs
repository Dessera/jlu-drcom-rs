pub mod cli;
pub mod config;
pub mod error;
pub mod interface;

pub fn ror(data: &[u8], pwd: &[u8]) -> Vec<u8> {
  let mut ret = Vec::new();
  for i in 0..pwd.len() {
    let x = data[i] ^ pwd[i];
    ret.push((x << 3) + (x >> 5));
  }
  ret
}

pub fn checksum(data: Vec<u8>) -> [u8; 4] {
  let mut sum = [0u8; 4];
  let len = data.len();
  let mut i = 0;
  while i + 3 < len {
    sum[0] ^= data[i + 3];
    sum[1] ^= data[i + 2];
    sum[2] ^= data[i + 1];
    sum[3] ^= data[i];
    i += 4;
  }
  if i < len {
    let mut tmp = [0u8; 4];
    for j in (0..4).rev() {
      tmp[j] = data[i];
      i += 1;
    }
    for j in 0..4 {
      sum[j] ^= tmp[j];
    }
  }
  let mut big_integer = num_bigint::BigUint::from_bytes_le(&sum);
  big_integer *= 1968u32;
  // let big_integer = big_integer & 0xff_ff_ff_ffu32;
  // let bytes = big_integer.to_bytes_le();
  let bytes = big_integer
    .to_bytes_le()
    .iter()
    .map(|x| (x & 0xff) as u8)
    .collect::<Vec<_>>();
  let mut i = 0;
  let mut ret = [0u8; 4];
  for j in (0..4).rev() {
    ret[j] = bytes[i];
    i += 1;
  }
  ret
}
