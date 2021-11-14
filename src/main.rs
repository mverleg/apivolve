use ::std::path::PathBuf;
use std::process::exit;

use ::env_logger;
use ::structopt::StructOpt;

use ::apivolve::{apivolve_check, apivolve_generate, apivolve_list, apivolve_next};
use apivolve::ApivResult;

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
    let mut dirs = collect_directories(&args.evolution_dirs);
    match args.cmd {
        Cmd::Check { .. } => apivolve_check(dirs),
        Cmd::Gen { .. } => apivolve_generate(dirs),
        Cmd::List { .. } => apivolve_list(dirs),
        Cmd::New { .. } => apivolve_next(dirs),
    }?;
    Ok(())
}

fn collect_directories(evolution_dirs: &[String]) -> Vec<PathBuf> {
    if evolution_dirs.is_empty() {
        return vec![PathBuf::from(DEFAULT_EVOLUTION_DIR)];
    }
    let mut paths = vec![];
    for evolution_path in evolution_dirs {
        for evolution_dir in evolution_path.split(";") {
            paths.push(PathBuf::from(evolution_dir))
        }
    }
    return paths
}
