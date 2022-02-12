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

use ::apivolve::{apivolve_check, apivolve_next, apivolve_release};
use ::apivolve::ApivResult;
use ::apivolve::gen1;
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

async fn run(args: &Args) -> ApivResult<()> {
    let mut dir = PathBuf::from(&args.evolution_dir);
    match &args.cmd {
        Cmd::Check { .. } => apivolve_check(dir).await?,
        Cmd::Gen { targets: Targets::List { json1 } } => {
            let list = gen1::apivolve_list_generators().await?;
            if *json1 {
                println!("{}", serde_json::to_string_pretty(&list).unwrap())
            } else {
                println!("{}", list)
            }
        },
        Cmd::Gen { targets: Targets::External(targets) } if targets.is_empty() => {
            eprintln!("expected at least one generation target");  // prevented by structopt
        }
        Cmd::Gen { targets: Targets::External(targets) } => println!("{}", gen1::apivolve_generate(dir, &*targets).await?),
        Cmd::List { json1 } => {
            let listing = list1::apivolve_list(dir).await?;
            if *json1 {
                println!("{}", serde_json::to_string_pretty(&listing).unwrap())
            } else {
                print!("{}", listing)
            }
        },
        Cmd::New { .. } => apivolve_next(dir).await?,
        Cmd::Release { .. } => apivolve_release(dir).await?,
    };
    Ok(())
}
