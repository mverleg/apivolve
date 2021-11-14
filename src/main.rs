use std::path::PathBuf;

use ::env_logger;
use ::structopt::StructOpt;

use crate::cli::args::Args;
use crate::cli::args::Cmd;
use crate::cli::args::DEFAULT_MIGRATION_DIR;

mod cli;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;


fn main() {
    env_logger::init();
    let args = Args::from_args();
    // info!("{:?}", "And every where that Mary went");
    run(&args);
}

fn run(args: &Args) {
    let mut dirs = collect_directories(&args.migration_dirs);
    //TODO @mark: split on ";" if only one arg
    println!("dirs: {:?}", dirs);  //TODO @mark: TEMPORARY! REMOVE THIS!
    match args.cmd {
        Cmd::Check { .. } => {
            unimplemented!()  //TODO @mark:
        }
        Cmd::Gen { .. } => {
            unimplemented!()  //TODO @mark:
        }
        Cmd::List { .. } => {
            unimplemented!()  //TODO @mark:
        }
        Cmd::New { .. } => {
            unimplemented!()  //TODO @mark:
        }
    }
}

fn collect_directories(migration_dirs: &[String]) -> Vec<PathBuf> {
    if migration_dirs.is_empty() {
        return vec![PathBuf::from(DEFAULT_MIGRATION_DIR)];
    }
    let mut paths = vec![];
    for migration_dir in migration_dirs {
        if migration_dir.contains(";") {
            unimplemented!();
        } else {
            paths.push(PathBuf::from(migration_dir))
        }
    }
    return paths
}
