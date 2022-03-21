use core::panic;
use std::env;
use std::process::Command;

fn main() {
    const OS: &str = env::consts::OS;
    let command = match OS {
        "linux" => "poweroff",
        "macos" => "poweroff",
        "windows" => "shutdown /p",
        _ => "",
    };
    run_command(command.to_string());
}

fn run_command(command: String) -> String {
    if command.is_empty() {
        panic!("failed to recognize your operating system")
    }
    let output = Command::new(command).output().expect("");
    let stdout = String::from_utf8(output.stdout);

    match stdout {
        Err(_e) => String::from(""),
        Ok(out) => out,
    }
}
