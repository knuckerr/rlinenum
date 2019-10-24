use crate::inotify::events::EVENTS;
use crossbeam_channel::{Receiver,Sender};
use failure::Error;
use nix::sys::inotify::{AddWatchFlags, InitFlags, Inotify, InotifyEvent};
use std::ffi::OsString;
use std::thread;
use walkdir::WalkDir;

pub struct Watcher {
    pub event: String,
    pub filename: String,
    pub path: String,
}

pub fn get_event(event: AddWatchFlags) -> String {
    let name_event = EVENTS.get(&event).unwrap_or(&"Error: Reading the Event");
    name_event.to_string()
}

pub fn read_events(events: Vec<InotifyEvent>, path_folder: &str, sender: Sender<Watcher>) {
    for event in events {
        let event_name = get_event(event.mask);
        let file_name = event
            .name
            .unwrap_or_else(|| OsString::from("Empty"))
            .into_string()
            .unwrap();
        let watcher = Watcher {
            event: event_name,
            filename: file_name,
            path: path_folder.to_string(),
        };
        sender.send(watcher).unwrap();
    }
}
pub fn walk_dir(paths: Vec<&str>) -> Vec<String> {
    let mut folders = Vec::new();
    for path in paths {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.path().is_dir() {
                let folder: String = entry.path().to_string_lossy().into();
                folders.push(folder);
            }
        }
    }
    folders
}
pub fn inotify_print(r: &Receiver<Watcher>,s:&Sender<Watcher>) {
    let data = r.recv();
    match data {
        Ok(data) => {
            if data.event == "CREATE DIR" {
                let folder = format!("{}/{}", data.path, data.filename);
                let result = start(&folder, s.clone());

                if result.is_err() {
                    eprintln!("{}", result.unwrap_err());
                }
            }
            println!(
                "PATH:{},FILENAME:{},EVENT:\t{}",
                data.path, data.filename, data.event
            );
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
pub fn start(path: &str, sender: Sender<Watcher>) -> Result<(), Error> {
    let instance = Inotify::init(InitFlags::empty()).unwrap();
    instance.add_watch(path, AddWatchFlags::IN_ALL_EVENTS)?;
    let path_clone = path.to_string().clone();
    thread::spawn(move || loop {
        let events = instance.read_events().unwrap();
        read_events(events, &path_clone, sender.clone());
    });
    Ok(())
}
