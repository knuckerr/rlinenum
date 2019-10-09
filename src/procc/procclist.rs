use failure::{err_msg, Error};
use std::collections::HashMap;
use std::fs::{self, ReadDir};
use std::path::Path;

pub struct Event {
    uid: i32,
    pid: i32,
    cmd: String,
}

pub struct Events {
    cmds: HashMap<i32, Event>,
}

pub fn refresh(events: &mut Events) -> Result<&mut Events, Error> {
    let pids = getpids()?;
    for _pid in pids {
        if !events.cmds.contains_key(&_pid) {
            let _cmd = proc_cmd_read(_pid);
            let _uuid = get_uid(_pid);
            if _cmd.is_ok() && _uuid.is_ok() {
                let event = Event {
                    pid: _pid,
                    cmd: _cmd?,
                    uid: _uuid?,
                };
                if event.cmd != "" {
                    println!(
                        "PID: {},UID:{}\t || CMD:\t{}",
                        event.pid, event.uid, event.cmd
                    );
                    events.cmds.insert(_pid, event);
                }
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
    let contents = fs::read_to_string(status_file)?;
    Ok(contents)
}

pub fn proc_cmd_read(id: i32) -> Result<String, Error> {
    let status_file = format!("/proc/{}/cmdline", id);
    let contents = fs::read_to_string(status_file)?;
    let contents = contents.replace("\u{0}", " ");
    Ok(contents)
}


pub fn getpids() -> Result<Vec<i32>, Error> {
    fs::read_dir("/proc")?
        .filter_map(|maybe_entry| {
            match maybe_entry {
                Ok(dir_entry) => {
                    let path = dir_entry.path();
                    let stem = path.file_stem()?.to_str()?;

                    match stem.parse::<i32>() {
                        Ok(pid) => Some(Ok(pid)),
                        Err(..) => None,
                    }
                },
                Err(e) => Some(Err(e.into())),
            }
        })
        .collect()
}

pub fn get_uid(pid: i32) -> Result<i32, Error> {
    let status = proc_status_read(pid)?;
    let lines: Vec<&str> = status.split('\n').collect();
    if lines.len() < 9 {
        err_msg("No uuid found");
    }
    let uuid1: Vec<&str> = lines[8].split('\t').collect();
    if uuid1.len() < 2 {
        err_msg("No uuid found");
    }
    let uuid = uuid1[1].parse::<i32>()?;

    Ok(uuid)
}

pub fn start() -> Result<(), Error> {
    let pids = getpids()?;
    let mut events = HashMap::new();
    for _pid in pids {
        let _cmd = proc_cmd_read(_pid)?;
        let _uuid = get_uid(_pid)?;
        let event = Event {
            pid: _pid,
            cmd: _cmd,
            uid: _uuid,
        };
        println!(
            "PID: {},UID:{}\t || CMD:\t{}",
            event.pid, event.uid, event.cmd
        );
        events.insert(_pid, event);
    }
    let mut all_events = Events { cmds: events };
    loop {
        refresh(&mut all_events)?;
    }
}
