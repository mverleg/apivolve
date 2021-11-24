#![allow(unused)] //TODO temporary

use ::std::path::PathBuf;
use ::std::process::exit;

use ::env_logger;
use ::structopt::StructOpt;

use ::apivolve::{apivolve_check, apivolve_generate, apivolve_list, apivolve_next, apivolve_release};
use ::apivolve::ApivResult;

use crate::cli::args::Args;
use crate::cli::args::Cmd;
use crate::cli::args::DEFAULT_EVOLUTION_DIR;

mod cli;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    env_logger::init();
    let args = Args::from_args();
    // info!("{:?}", "And every where that Mary went");
    if let Err(err) = run(&args) {
        eprintln!("{}", err);
        exit(1)
    }
}

pub fn run(args: &Args) -> ApivResult<()> {
    let mut dir = PathBuf::from(&args.evolution_dir);
    match args.cmd {
        Cmd::Check { .. } => apivolve_check(dir),
        Cmd::Gen { .. } => apivolve_generate(dir),
        Cmd::List { .. } => apivolve_list(dir),
        Cmd::New { .. } => apivolve_next(dir),
        Cmd::Release { .. } => apivolve_release(dir),
    }?;
    Ok(())
}
