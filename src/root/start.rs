use crate::cmds::run::{run, Root};
use crate::export::report::{export_cmd_list, export_results, json_report};
use crate::procc::procclist::{start, Event};
use crate::tools::extra::http_get;
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
                println!("{}", data.cmd_s);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}

pub fn begin(cmds: &mut Root, args: ArgMatches) -> Result<(),Error> {
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
    let echo = args.value_of("echo").unwrap_or("none");
    Ok(())
}
