use ::env_logger;
use ::structopt::StructOpt;

use crate::args::{Args, Cmd};

mod args;

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
