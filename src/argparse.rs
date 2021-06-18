use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "gpu-info", about = "A list of command line flags.")]
pub struct Opt {
    /// Prints GPU information to terminal every second
    #[structopt(short = "w", long = "watch", about = "watch over updated metrics.")]
    pub watch: bool,
}

pub fn argparse() -> Opt {
    let opt = Opt::from_args();
    return opt;
}