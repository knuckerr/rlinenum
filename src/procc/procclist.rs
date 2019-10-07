use failure::{err_msg, Error};
use std::fs::File;
use std::fs::ReadDir;
use std::io::prelude::*;
use std::path::Path;

pub fn read_proc_dir() -> Result<ReadDir, Error> {
    let dir = Path::new("/proc").read_dir()?;
    Ok(dir)
}

pub fn proc_status_read(id: i32) -> Result<String, Error> {
    let status_file = format!("/proc/{}/status", id);
    let mut file = File::open(status_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn proc_cmd_read(id: i32) -> Result<String, Error> {
    let status_file = format!("/proc/{}/cmdline", id);
    let mut file = File::open(status_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn f2pid(folder: &Path) -> Result<i32, Error> {
    let get_pid = folder
        .file_stem()
        .ok_or(err_msg("Failed to find file stem"))?
        .to_str()
        .ok_or(err_msg("Failed to create string"))?;
    let pid = get_pid.parse::<i32>()?;
    Ok(pid)
}

pub fn getpids() -> Result<Vec<i32>, Error> {
    let mut pids = Vec::new();
    let procc_list = read_proc_dir()?;
    for procc in procc_list {
        let procc = procc?;
        let path = procc.path();
        if path.is_dir() {
            let pid = f2pid(path.as_path());
            if pid.is_ok() {
                pids.push(pid?);
            }
        }
    }
    Ok(pids)
} 
