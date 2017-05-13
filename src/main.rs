#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate log;

extern crate regex;
use regex::Regex;


use std::process::Command;
use toml::Value;
use std::fs::File;
use std::io::Read;
use std::ffi::OsStr;

#[derive(Deserialize)]
struct Config {
    ip_rang: String,
    api_key: String,
    channel_name: String,
    str_enter: String,
    str_leave: String,
}

fn parseconfig(path: &str) -> Config {
    let mut data = String::new();
    let mut f = File::open(path).expect("Unable to open file");
    f.read_to_string(&mut data)
        .expect("Unable to read string");


    let conf: Config = toml::from_str(&data).unwrap();
    //let value = data.parse::<Value>().unwrap();
    //let iprang = value["ip_rang"].as_str().unwrap();
    //return iprang.to_string();
    return conf;
}

fn nmac(conf: Config) -> Vec<String> {
    //let ipref = &[" ", &conf.ip_rang].concat();
    let ipr = OsStr::new(&conf.ip_rang);
    let output = Command::new("nmap")
        .arg("-sP")
        .arg("-n")
        .arg(ipr)
        .output()
        .expect("failed to execute process");

    //let output = Command::new("uname")
    //  .arg("-a")
    // .output()
    // .expect("failed to execute process");
    let out = String::from_utf8_lossy(&output.stdout);
    //println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    let re = Regex::new(r"([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})").unwrap();
    let mut macad = vec![];
    for caps in re.captures_iter(&out) {
        //println!("{:}", caps.get(0).unwrap().as_str());
        macad.push(caps.get(0).unwrap().as_str().to_string());
    }

    return macad;
}

fn main() {
    //println!("{}", parsefiles("config.example.toml"));
    println!("Hello, world!");
    let conf = parseconfig("config.example.toml");
    let macad = nmac(conf);
    println!("{:?}", macad);
}
