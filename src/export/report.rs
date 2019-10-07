use crate::cmds::run::Root;
use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

pub fn report(path: &str) -> Result<File, Box<dyn Error>> {
    if Path::new(path).exists() {
        fs::remove_file(path)?;
    }
    let path = Path::new(path);
    fs::create_dir_all(path.parent().unwrap())?;
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create_new(true)
        .open(path)?;
    Ok(file)
}

pub fn json_report(cmds: &Root, path: &str) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string(cmds)?;
    let mut report = report(path)?;
    report.write_all(json.as_bytes())?;
    Ok(())
}
