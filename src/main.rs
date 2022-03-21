use std::env;

use std::net::TcpListener;
use std::process::Command;

use structopt::StructOpt;
use tcpts::Opt;

fn main() {
    const OS: &str = env::consts::OS;
    let command = match OS {
        "linux" => "poweroff",
        "macos" => "poweroff",
        "windows" => "shutdown/s",
        _ => "",
    };

    tcpts::run_command(command.to_string());

    let opt = Opt::from_args();
    let listener = TcpListener::bind(String::from("127.0.0.1:") + &opt.port);
    if listener.is_ok() {
        let listener = listener.unwrap();
        for _ in listener.incoming() {
            tcpts::run_command(command.to_string());
        }
    } else {
        println!("Port not supported");
    }
}
