use core::panic;
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
        "windows" => "shutdown /p",
        _ => "",
    };
    let opt = Opt::from_args();
    let listener = TcpListener::bind(String::from("127.0.0.1:") + &opt.port);

    if listener.is_ok() {
        let listener = listener.unwrap();
        for _ in listener.incoming() {
            run_command(command.to_string());
        }
    } else {
        println!("Port not supported");
    }
}

fn run_command(command: String) -> String {
    if command.is_empty() {
        panic!("Failed to recognize your operating system")
    }
    let output = Command::new(command).output().expect("");
    let stdout = String::from_utf8(output.stdout);

    match stdout {
        Err(_e) => String::from(""),
        Ok(out) => out,
    }
}
