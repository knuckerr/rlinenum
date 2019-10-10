use failure::{err_msg, Error};
use std::collections::HashMap;
use std::fs::{self, ReadDir};
use std::path::Path;


#[derive(Clone)]
pub struct Event {
    uid_s: i32,
    pid_s: i32,
    cmd_s: String,
}

#[derive(Clone)]
pub struct Events {
    pids: HashMap<i32, Event>,
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
        .filter_map(|maybe_entry| match maybe_entry {
            Ok(dir_entry) => {
                let path = dir_entry.path();
                let stem = path.file_stem()?.to_str()?;

                match stem.parse::<i32>() {
                    Ok(pid) => Some(Ok(pid)),
                    Err(..) => None,
                }
            }
            Err(e) => Some(Err(e.into())),
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

pub fn get_events() -> Result<Events, Error> {
    let mut events = HashMap::new();
    let pids = getpids()?;
    for pid_t in pids {
        let event = pid2event(pid_t)?;
        events.insert(pid_t, event);
    }
    Ok(Events { pids: events })
}

pub fn pid2event(pid: i32) -> Result<Event, Error> {
    let cmd = proc_cmd_read(pid)?;
    let uid = get_uid(pid)?;
    let event = Event {
        pid_s: pid,
        uid_s: uid,
        cmd_s: cmd,
    };
    Ok(event)
}

pub fn clear(events:&mut Events) -> Result<&mut Events,Error> {
    let pids = getpids()?;
    events.pids.retain(|k, _| pids.contains(k));
    Ok(events)
}

pub fn refresh(events: &mut Events) -> Result<&mut Events, Error> {
    let new_pids = getpids()?;
    for pid in new_pids {
        if !events.pids.contains_key(&pid) {
            let event = pid2event(pid);
            if event.is_ok() {
                let event = event.unwrap();
                if event.cmd_s != "" {
                    println!(
                        "PID: {},UID:{}\t || CMD:\t{}",
                        event.pid_s, event.uid_s, event.cmd_s
                    );
                    events.pids.insert(pid, event);
                }
            }
        }
    }

    Ok(events)
}

pub fn start_watch() -> Result<(), Error> {
    let mut events = get_events()?;
    for (_pid, event) in &events.pids {
        println!(
            "pid: {},uid:{}\t || cmd:\t{}",
            event.pid_s, event.uid_s, event.cmd_s
        );
    }
    loop {
        refresh(&mut events)?;
    }
}
