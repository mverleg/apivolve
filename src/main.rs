#![allow(unused)] //TODO temporary

use ::std::borrow::Borrow;
use ::std::path;
use ::std::path::PathBuf;
use ::std::process::exit;

use ::env_logger;
use ::futures::executor::block_on;
use ::lazy_static::lazy_static;
use ::regex::Regex;
use ::structopt::StructOpt;
use ::which::which_re;

use ::apivolve::{
    apivolve_check, apivolve_generate, apivolve_next, apivolve_release,
};
use ::apivolve::ApivResult;
use ::apivolve::list1;

use crate::cli::args::{Args, Targets};
use crate::cli::args::Cmd;
use crate::cli::args::DEFAULT_EVOLUTION_DIR;

mod cli;

lazy_static! {
    static ref GEN_EXE_RE: Regex = Regex::new("^apivolve0-gen-.*").unwrap();
}

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
        Cmd::Gen {
            targets: Some(Targets::Targets(targets)),
        } => apivolve_generate(dir, targets).await,
        Cmd::Gen { targets: None } => apivolve_list_generators(dir).await,
        Cmd::List { json1 } => {
            let listing = list1::apivolve_list(dir).await?;
            if *json1 {
                println!("{}", listing.to_string())
            } else {
                print!("{}", listing)
            }
            Ok(())
        },
        Cmd::New { .. } => apivolve_next(dir).await,
        Cmd::Release { .. } => apivolve_release(dir).await,
    }
}

pub fn which_re2(regex: impl Borrow<Regex>) -> Result<(), String> {
    let r: &Regex = regex.borrow();
    Ok(())
}

pub async fn apivolve_list_generators(evolution_dir: PathBuf) -> ApivResult<()> {
    which_re2(Regex::new("abc").unwrap()); //TODO @mark: TEMPORARY! REMOVE THIS!
    which_re2(&Regex::new("abc").unwrap()); //TODO @mark: TEMPORARY! REMOVE THIS!
    match which_re2(&*GEN_EXE_RE) {
        Ok(_) => Ok(()),
        Err(err) => Err("failed to scan $PATH for apivolve generators".to_owned()),
    }
}
