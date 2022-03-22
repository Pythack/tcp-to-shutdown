use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "TCP to shutdown",
    about = "Shutdown a computer if a specified port is requested"
)]
pub struct Opt {
    #[structopt(help = "Specify the port to listen on", default_value = "80")]
    pub port: String,

    #[structopt(
        short,
        long,
        help = "Specify the delay between request and shutdown in seconds",
        default_value = "0"
    )]
    pub delay: u64,
}
