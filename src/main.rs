use ::env_logger;
use ::structopt::StructOpt;

use crate::cli::args::Args;
use crate::cli::args::Cmd;

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
    let mut dirs = &args.migration_dirs;
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
