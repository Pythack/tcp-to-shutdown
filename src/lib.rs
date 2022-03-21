use core::panic;
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "TCP to shutdown",
    about = "Shutdown a computer if a specified port is requested"
)]
pub struct Opt {
    #[structopt(help = "Specify the port to listen on", default_value = "80")]
    pub port: String,
}

pub fn run_command(command: String) {
    if command.is_empty() {
        panic!("Failed to recognize your operating system")
    }
    let _output = Command::new(command).output();
}
