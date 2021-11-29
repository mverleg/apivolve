#![allow(unused)] //TODO temporary

use ::std::path::PathBuf;
use ::std::process::exit;

use ::env_logger;
use ::structopt::StructOpt;

use ::apivolve::{apivolve_check, apivolve_generate, apivolve_list, apivolve_next, apivolve_release};
use ::apivolve::ApivResult;
use ::futures::executor::block_on;

use crate::cli::args::{Args, Targets};
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
    if let Err(err) = block_on(run(&args)) {
        eprintln!("{}", err);
        exit(1)
    }
}

pub async fn run(args: &Args) -> ApivResult<()> {
    let mut dir = PathBuf::from(&args.evolution_dir);
    match &args.cmd {
        Cmd::Check { .. } => apivolve_check(dir).await,
        Cmd::Gen { targets: Some(Targets::Targets(targets)) } => apivolve_generate(dir, targets).await,
        Cmd::Gen { targets: None } => apivolve_list_generators(dir).await,
        Cmd::List { .. } => apivolve_list(dir).await,
        Cmd::New { .. } => apivolve_next(dir).await,
        Cmd::Release { .. } => apivolve_release(dir).await,
    }
}


pub async fn apivolve_list_generators(evolution_dir: PathBuf) -> ApivResult<()> {
    unimplemented!() //TODO @mark: TEMPORARY! REMOVE THIS!
}
