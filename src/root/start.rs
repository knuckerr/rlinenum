use crate::cmds::run::{run, Root};
use crate::export::report::{export_cmd_list, export_results, json_report};
use crate::inotify::inotify::start as i_start;
use crate::inotify::inotify::{walk_dir, Watcher};
use crate::procc::procclist::{start, Event};
use crate::tools::extra::{http_get, tcp_echo};
use clap::ArgMatches;
use crossbeam_channel::unbounded;
use crossbeam_channel::{Receiver, Sender};
use failure::Error;

fn start_enum(cmds: &mut Root, report: &str, jreport: &str) -> Result<(), Error> {
    for type_cmd in cmds.cmds.iter_mut() {
        println!("{}", type_cmd.print);
        for cmd in type_cmd.cmds.iter_mut() {
            run(cmd, &cmds.binary_list);
            if cmd.results != "" {
                println!("{}", cmd.print);
                println!("{}", cmd.results);
            }
        }
    }
    if report != "none" {
        export_results(cmds, report)?;
    }
    if jreport != "none" {
        json_report(cmds, jreport)?;
    }
    Ok(())
}

fn start_procc() {
    let (s, r): (Sender<Event>, Receiver<Event>) = unbounded();
    start(s);
    loop {
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
}

fn start_watch(folders: Vec<&str>, s: Sender<Watcher>) {
    for f in folders {
        let result = i_start(f, s.clone());
        if result.is_err() {
            eprintln!("{}", result.unwrap_err());
        }
    }
}

fn start_watch_walker(folders: Vec<&str>, s: Sender<Watcher>) {
    let walker = walk_dir(folders);
    for f in walker {
        let result = i_start(&f, s.clone());
        if result.is_err() {
            eprintln!("{}", result.unwrap_err());
        }
    }
}

fn watcher(folders: Vec<&str>, folders_walk: Vec<&str>) {
    let (s, r): (Sender<Watcher>, Receiver<Watcher>) = unbounded();
    if folders.len() > 0 {
        start_watch(folders, s.clone());
    }
    if folders_walk.len() > 0 {
        start_watch_walker(folders_walk, s.clone());
    }
    loop {
        let data = r.recv();
        match data {
            Ok(data) => {
                if data.event == "CREATE DIR" {
                    let folder = format!("{}/{}", data.path, data.filename);
                    let result = i_start(&folder, s.clone());

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
}

pub fn begin(cmds: &mut Root, args: ArgMatches) -> Result<(), Error> {
    let folders: Vec<_> = args.values_of("watch").into_iter().flatten().collect();
    let folders_walk: Vec<_> = args.values_of("walker").into_iter().flatten().collect();
    if folders_walk.len() > 0 || folders.len() > 0 {
        watcher(folders, folders_walk);
    }
    if args.is_present("enum") {
        let report = args.value_of("report").unwrap_or("none");
        let jreport = args.value_of("jreport").unwrap_or("none");
        start_enum(cmds, report, jreport)?;
    }
    if args.is_present("pss") {
        start_procc();
    }
    let cmds_enum = args.value_of("cmds_enum").unwrap_or("none");
    if cmds_enum != "none" {
        export_cmd_list(cmds, cmds_enum)?;
    }
    let url = args.value_of("http_get").unwrap_or("none");
    if url != "none" {
        http_get(url)?;
    }
    if let Some(args) = args.subcommand_matches("echo") {
        let ip = args.value_of("ip").unwrap_or("127.0.0.1");
        let port = args
            .value_of("port")
            .unwrap_or("443")
            .parse::<i32>()
            .unwrap();
        let file = args.value_of("file").unwrap();
        tcp_echo(ip, port, file)?;
    }

    Ok(())
}
