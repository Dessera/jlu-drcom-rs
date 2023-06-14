use std::fs::File;

use super::error::DrResult;
pub fn get_logfile() -> DrResult<File> {
  let tmp_dir = std::env::temp_dir();
  let current_time = chrono::Local::now().format("%Y%m%d-%H:%M:%S");
  let tmp_file_name = format!("jlu_drcom_rs-{}.log", current_time);
  let tmp_file_path = tmp_dir.join(tmp_file_name);
  File::create(tmp_file_path).map_err(|e| e.into())
}

pub fn remove_all_logfiles() -> DrResult<()> {
  let tmp_dir = std::env::temp_dir();
  let tmp_files = std::fs::read_dir(tmp_dir)?;
  for tmp_file in tmp_files {
    let tmp_file = tmp_file?;
    let tmp_file_path = tmp_file.path();
    if tmp_file_path.is_file() {
      let tmp_file_name = tmp_file_path.file_name().unwrap().to_str().unwrap();
      if tmp_file_name.starts_with("jlu_drcom_rs-") && tmp_file_name.ends_with(".log") {
        std::fs::remove_file(tmp_file_path)?;
      }
    }
  }
  Ok(())
}
