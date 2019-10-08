use failure::{err_msg, Error};
use std::collections::HashMap;
use std::fs::File;
use std::fs::ReadDir;
use std::io::prelude::*;
use std::path::Path;

pub struct Event {
    uuid: i32,
    pid: i32,
    cmd: String,
}

pub struct Events {
    cmds: HashMap<i32, Event>,
}

pub fn refresh(events: &mut Events) -> Result<&mut Events, Error> {
    let pids = getpids()?;
    for pid in pids {
        if !events.cmds.contains_key(&pid) {
            let _cmd = proc_cmd_read(pid);
            let _uuid = get_uuid(pid);
            if _cmd.is_ok() && _uuid.is_ok() {
                let event = Event {
                    pid: pid,
                    cmd: _cmd?,
                    uuid: _uuid?,
                };
                println!("PID: {},UID:{}\t || CMD:\t{}", event.pid, event.uuid, event.cmd);
                events.cmds.insert(pid, event);
            }
        }
    }
    Ok(events)
}
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
    let contents = contents.replace("\u{0}"," ");
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

pub fn get_uuid(pid: i32) -> Result<i32, Error> {
    let status = proc_status_read(pid)?;
    let lines: Vec<&str> = status.split("\n").collect();
    if lines.len() < 9 {
        err_msg("No uuid found");
    }
    let uuid1: Vec<&str> = lines[8].split("\t").collect();
    if uuid1.len() < 2 {
        err_msg("No uuid found");
    }
    let uuid = uuid1[1].parse::<i32>()?;

    Ok(uuid)
}

pub fn start() -> Result<(), Error> {
    let pids = getpids()?;
    let mut events = HashMap::new();
    for pid in pids {
        let _cmd = proc_cmd_read(pid)?;
        let _uuid = get_uuid(pid)?;
        let event = Event {
            pid: pid,
            cmd: _cmd,
            uuid: _uuid,
        };
        println!("PID: {},UID:{}\t || CMD:\t{}", event.pid, event.uuid, event.cmd);
        events.insert(pid, event);
    }
    let mut all_events = Events { cmds: events };
    loop {
        refresh(&mut all_events)?;
    }
}
