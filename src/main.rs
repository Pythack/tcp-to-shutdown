use std::{thread, time};

use std::net::TcpListener;
use system_shutdown::shutdown;

use structopt::StructOpt;
use tcpts::Opt;

fn main() {
    let opt = Opt::from_args();
    let listener = TcpListener::bind(String::from("127.0.0.1:") + &opt.port);
    if listener.is_ok() {
        let listener = listener.unwrap();
        for _ in listener.incoming() {
            println!("Received request");
            thread::sleep(time::Duration::from_secs(opt.delay));
            match shutdown() {
                Ok(_) => println!("Shutting down..."),
                Err(error) => eprintln!("Failed to shut down: {}", error),
            };
        }
    } else {
        println!("Port not supported");
    }
}
