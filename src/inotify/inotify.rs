use crate::inotify::events::EVENTS;
use crossbeam_channel::Sender;
use nix::sys::inotify::{AddWatchFlags, InitFlags, Inotify, InotifyEvent};
use std::ffi::OsString;
use std::thread;

pub struct Watcher {
    event: String,
    filename: String,
    path: String,
}

pub fn get_event(event: AddWatchFlags) -> String {
    let name_event = EVENTS.get(&event).unwrap_or(&"Error: Reading the Event");
    name_event.to_string()
}

pub fn read_events(events: Vec<InotifyEvent>, path: &str, sender: Sender<Watcher>) {
    for event in events {
        let event_name = get_event(event.mask);
        let filename = event
            .name
            .unwrap_or_else(|| OsString::from("Empty"))
            .into_string()
            .unwrap();
        let watcher = Watcher {
            event: event_name,
            filename: filename,
            path: path.to_string(),
        };
        sender.send(watcher).unwrap();
    }
}
pub fn start(path: &'static str, sender: Sender<Watcher>) {
    let instance = Inotify::init(InitFlags::empty()).unwrap();
    instance
        .add_watch(path, AddWatchFlags::IN_ALL_EVENTS)
        .unwrap();
    thread::spawn(move || loop {
        let events = instance.read_events().unwrap();
        read_events(events, path, sender.clone());
    });
}
