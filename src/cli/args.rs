use ::std::path::PathBuf;
use ::structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "planr", about = "Simple CLI to interact with Planr.")]
pub struct Args {
    #[structopt(long = "migration-dir", short = "d", parse(from_os_str), default_value="./apivolve", env = "APIVOLVE_MIGRATION_PATH")]
    pub migration_dirs: Vec<PathBuf>,
    #[structopt(subcommand)]
    pub cmd: Cmd,
}

#[derive(Debug, StructOpt)]
pub enum Cmd {
    #[structopt(about = "Check that the API can be upgraded by running all migrations")]
    Check {
    },
    #[structopt(about = "Generate the API code for a specific target(s)")]
    Gen {
        #[structopt(required = true)]
        target: Vec<String>,
    },
    #[structopt(about = "List all the migrations in valid resolution order")]
    List {
    },
    #[structopt(about = "Create a new migration file at the head of the current chain")]
    New {
    },
}
