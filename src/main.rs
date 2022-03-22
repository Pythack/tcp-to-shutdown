use std::{thread, time};

use std::net::TcpListener;
use system_shutdown::shutdown;

use structopt::StructOpt;
use tcpts::Opt;

fn main() {
    let opt = Opt::from_args();
    let listener = TcpListener::bind(String::from("127.0.0.1:") + &opt.port);
    if let Ok(listener) = listener {
        println!("\x1b[94mListening on port {}\x1b[0m", &opt.port);
        for _ in listener.incoming() {
            println!("\x1b[94mReceived request\x1b[0m");
            thread::sleep(time::Duration::from_secs(opt.delay));
            match shutdown() {
                Ok(_) => println!("Shutting down..."),
                Err(error) => eprintln!("Failed to shut down: {}", error),
            };
        }
    } else {
        println!("\x1b[91mPort not supported\x1b[0m");
    }
}
