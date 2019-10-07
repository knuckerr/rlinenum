use std::net::TcpStream;
use std::fs;
use std::error::Error;
use std::io::{Write,copy};
use std::fs::File;

pub fn tcp_echo(ip:&str,port:i32,report:&str) -> Result<(),Box<dyn Error>>{
    let ip_port = format!("{}:{}",ip,port);
    let contents = fs::read_to_string(report)?;
    let mut stream = TcpStream::connect(ip_port)?;
    let _ = stream.write(&contents.as_bytes());
    Ok(())
}

pub fn http_get(url:&str) -> Result<(),Box<dyn Error>>{
    let mut resp = reqwest::get(url).expect("request failed");
    let mut dest = {
        let fname = resp
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);
        let fname = format!("/tmp/{}",fname);
        println!("will be located under: '{:?}'", fname);
        File::create(fname)?
    };
    copy(&mut resp, &mut dest)?;
    Ok(())
}

