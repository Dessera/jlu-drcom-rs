pub mod cli;
pub mod error;
pub mod interface;

pub fn ror(data: &[u8], pwd: &[u8]) -> Vec<u8> {
  let mut ret = Vec::new();
  for i in 0..pwd.len() {
    let x = data[i] ^ pwd[i];
    ret.push(((x << 3) & 0xFF) + (x >> 5));
  }
  ret
}
