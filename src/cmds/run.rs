use serde::{Deserialize, Serialize};
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    #[serde(skip_serializing)]
    pub binary_list: String,
    pub cmds: Vec<Cmds>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cmds {
    #[serde(rename = "type")]
    pub type_field: String,
    pub print: String,
    pub cmds: Vec<Cmd>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cmd {
    pub cmd: String,
    pub print: String,
    #[serde(default)]
    pub results: String,
    #[serde(default)]
    #[serde(skip_serializing)]
    pub extra_cmds: Vec<String>,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub require: String,
}
impl Cmd {
    fn new(
        _cmd: String,
        _results: String,
        _print: String,
        _extra_cmds: Vec<String>,
        _require: String,
    ) -> Cmd {
        Cmd {
            cmd: _cmd,
            print: _print,
            results: _results,
            extra_cmds: _extra_cmds,
            require: _require,
        }
    }
}

pub fn run(cmd: &mut Cmd, binary_list: &str) {
    if cmd.require != "" {
        return;
    }
    let mut child_shell = Command::new("/bin/bash")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    if cmd.cmd.contains("$binarylist") {
        cmd.cmd = format!("export binary_list={}; {}", binary_list, cmd.cmd);
    }

    child_shell
        .stdin
        .as_mut()
        .ok_or("Child process stdin has not been captured!")
        .unwrap()
        .write_all(cmd.cmd.as_bytes())
        .unwrap();

    let output = child_shell.wait_with_output().unwrap();
    cmd.results = String::from_utf8_lossy(output.stdout.as_slice()).to_string();
}
