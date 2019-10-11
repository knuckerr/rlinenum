#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;

mod cmds;
mod export;
mod inotify;
mod procc;
mod tools;

fn main() {
    let _file = include_str!("../cmds.json");
    //http_get("http://localhost/test.txt").unwrap();
    //tcp_echo("localhost",443,"/tmp/test.txt").unwrap();
    /*
    for type_cmd in cmds.cmds.iter_mut() {
        println!("{}", type_cmd.print);
        for cmd in type_cmd.cmds.iter_mut() {
            run(cmd, &cmds.binary_list);
        }
    }
    // We create a new inotify instance.
    let inotify = init().unwrap();



    // We add a new watch on directory "test" for all events.
    add_watch(inotify,"/tmp").unwrap();

    let events = inotify.read_events().unwrap();
    read_events(events);
    */
}
