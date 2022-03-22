# tcpts

Shutdown computer on request on given port. Default is 80.

## Usage

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
