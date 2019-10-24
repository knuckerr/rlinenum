use crossbeam_channel::{Receiver, Sender};
use failure::{err_msg, Error};
use std::collections::HashMap;
use std::fs::{self, ReadDir};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
pub struct Event {
    pub uid_s: i32,
    pub pid_s: i32,
    pub cmd_s: String,
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

pub fn add_pid(pid: i32, sender: Sender<Event>, procc_list: &mut HashMap<i32, String>) {
    let cmd = proc_cmd_read(pid);
    let uid = get_uid(pid);
    match (cmd, uid) {
        (Ok(cmd), Ok(uid)) => {
            let event = Event {
                pid_s: pid,
                uid_s: uid,
                cmd_s: cmd.clone(),
            };
            sender.send(event).unwrap();
            procc_list.insert(pid, cmd);
        }
        _ => {}
    }
}

pub fn refresh(procc_list: &mut HashMap<i32, String>, sender: Sender<Event>) -> Result<(), Error> {
    let pids = getpids()?;
    for pid in pids {
        if !procc_list.contains_key(&pid) {
            add_pid(pid, sender.clone(), procc_list);
        }
    }
    Ok(())
}
pub fn print_ps(r: &Receiver<Event>) {
    let data = r.recv();
    match data {
        Ok(data) => {
            if data.cmd_s != "" {
                println!("PID:{},UID:{},CMD:\t{}", data.pid_s, data.uid_s, data.cmd_s);
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}

pub fn start(sender: Sender<Event>) {
    let procc_list: HashMap<i32, String> = HashMap::new();
    let list = Arc::new(Mutex::new(procc_list));
    let list_clone = list.clone();
    thread::spawn(move || loop {
        refresh(&mut list_clone.lock().unwrap(), sender.clone()).unwrap();
    });
}
