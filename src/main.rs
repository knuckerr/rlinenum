#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;

mod cmds;
mod export;
mod inotify;
mod procc;
mod tools;
mod root;
use root::start::begin;
use clap::{App, Arg};
use cmds::run::Root;

fn main() {
    let cmd_json = include_str!("../cmds.json");
    let mut cmds :Root= serde_json::from_str(cmd_json).unwrap();
    let matches = App::new("Rlinuem")
        .version("1.0")
        .author("Knucker")
        .about("Enumeration scaning && process watch && inotify folders watch")
        .arg(
            Arg::with_name("enum")
                .long("enum")
                .help("Begin the enumeration scaning")
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::with_name("pss")
                .long("pss")
                .help("Begin the process scaning")
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::with_name("report")
                .long("report")
                .short("r")
                .value_name("FILENAME")
                .help("export the enumeration scan results in txt")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("jreport")
                .long("jreport")
                .short("jr")
                .value_name("FILENAME")
                .help("export the enumeration scan results in json")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("cmds_enum")
                .long("cmds_enum")
                .short("cmde")
                .value_name("FILENAME")
                .help("export the commands that are been used in the enumeration scaning")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("http_get")
                .long("http_get")
                .short("get")
                .value_name("URL")
                .help("download the file from the url and save it in to the /tmp[File]")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("watch")
                .long("watch")
                .short("w")
                .value_name("PATH")
                .help("the path to start inotify watcher and print all the events")
                .takes_value(true)
                .multiple(true)
                .required(false),
        )
        .arg(
            Arg::with_name("walker")
                .long("walker")
                .value_name("PATH")
                .help("the path to start folder walker and for each sub folder start the inotify watcher and print all the events")
                .takes_value(true)
                .multiple(true)
                .required(false),
        )
        .arg(
            Arg::with_name("echo")
                .long("echo")
                .value_name("IP:PORT")
                .help("TCP ECHO a file to the nc listener")
                .takes_value(true)
                .multiple(true)
                .required(false),
        )
        .get_matches();
    begin(&mut cmds,matches);
}
