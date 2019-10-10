use crate::cmds::run::Root;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use failure::Error;

pub fn report(path: &str) -> Result<File,Error> {
    if Path::new(path).exists() {
        fs::remove_file(path)?;
    }
    let path = Path::new(path);
    fs::create_dir_all(path.parent().unwrap())?;
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create_new(true)
        .open(path)?;
    Ok(file)
}

pub fn json_report(cmds: &Root, path: &str) -> Result<(),Error> {
    let json = serde_json::to_string(cmds)?;
    let mut report = report(path)?;
    report.write_all(json.as_bytes())?;
    Ok(())
}

pub fn export_cmd_list(cmds:&Root,path:&str) -> Result<(),Error> {
    let mut file = report(path)?;
    for cmd_root in cmds.cmds.iter(){
        let cmd_root_write = format!("{}\n\n",cmd_root.print);
        file.write_all(cmd_root_write.as_bytes())?;
        for cmd in cmd_root.cmds.iter() {
            let cmd_print_write = format!("{}\n\n",cmd.print);
            file.write_all(cmd_print_write.as_bytes())?;
            let cmd_cmd_write = format!("{}\n\n",cmd.cmd);
            file.write_all(cmd_cmd_write.as_bytes())?;
        }
    }
    Ok(())
}
