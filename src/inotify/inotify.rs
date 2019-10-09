use crate::inotify::events::EVENTS;
use nix::sys::inotify::{AddWatchFlags, InitFlags, Inotify,InotifyEvent,WatchDescriptor};
use std::ffi::OsString;
use failure::Error;


pub fn get_event(event: AddWatchFlags) -> String {
    let name_event = EVENTS.get(&event).unwrap_or(&"Error: Reading the Event");
    name_event.to_string()
}

pub fn init() -> Result<Inotify,Error> {
    let instance = Inotify::init(InitFlags::empty())?;
    Ok(instance)
}

pub fn add_watch(inotify:Inotify,path:&str) -> Result<WatchDescriptor,Error> {
    let wd = inotify.add_watch(path, AddWatchFlags::IN_ALL_EVENTS)?;
    Ok(wd)
}

pub fn remove_watch(inotify:Inotify,wd:WatchDescriptor) -> Result<(),Error>{
    inotify.rm_watch(wd)?;
    Ok(())
}

pub fn read_events(events:Vec<InotifyEvent>) {
    for event in events{
        let event_name = get_event(event.mask);
        let filename = event.name.unwrap_or_else(|| OsString::from("Empty")).into_string().unwrap();
        let results = format!("Event: {}, Filename:{}",event_name,filename);
        println!("{}",results);
    }

}
