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
