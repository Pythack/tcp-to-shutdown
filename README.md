# tcpts

Shutdown computer on request on given port. Default is 80.

## Help

```console
TCP to shutdown 0.1.0
Shutdown a computer if a specified port is requested

USAGE:
    tcpts [OPTIONS] [port]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --delay <delay>    Specify the delay between request and shutdown in seconds [default: 0]

ARGS:
    <port>    Specify the port to listen on [default: 80]
```

## How to use it

Run the binary and then send a request to the specified localhost port (default: 80) to shut down
your PC. On macOS, you will need to run it as super user.

## Installation

### GNU/Linux

* `wget https://github.com/Pythack/tcpts/releases/latest/download/tcpts-x86_64-unknown-linux-gnu.tar.gz`
* `tar -xzvf tcpts-x86_64-unknown-linux-gnu.tar.gz`
* `sudo mv tcpts-x86_64-unknown-linux-gnu/tcpts /bin`

### MacOS

If you have an M1 cpu, your architecture is ARM. If you have an Intel cpu, your architecture is x86_64

#### ARM

* `wget https://github.com/Pythack/tcpts/releases/latest/download/tcpts-aarch64-apple-darwin.zip`
* `unzip tcpts-aarch64-apple-darwin.zip`
* `sudo mv tcpts-aarch64-apple-darwin/tcpts /bin`

#### x86_64

* `wget https://github.com/Pythack/tcpts/releases/latest/download/tcpts-x86_64-apple-darwin.zip`
* `unzip tcpts-x86_64-apple-darwin.zip`
* `sudo mv tcpts-x86_64-apple-darwin/tcpts /bin`

## Building

* As permscan is written in rust, you will need rust to build it. The
  [latest](https://www.rust-lang.org/tools/install) version is recommended.

* To build :

  * `git clone https://github.com/Pythack/permscan`
  * `cd permscan`
  * `cargo build --release`

## License

This project is licensed under both :

* The Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* The MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)
