#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;


mod tools;
mod export;
mod cmds;
mod inotify;
mod procc;
use procc::procclist::start;


fn main() {
    let file = include_str!("../cmds.json");
    //http_get("http://localhost/test.txt").unwrap();
    //tcp_echo("localhost",443,"/tmp/test.txt").unwrap();
    /*
    let mut cmds: Root = serde_json::from_str(&file).expect("sss");
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

    start();

}
