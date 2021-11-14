use ::structopt::StructOpt;

pub static DEFAULT_MIGRATION_DIR: &'static str = "./apivolve";

#[derive(Debug, StructOpt)]
#[structopt(name = "apivolve", about = "API evolution tool, it helps keep your APIs backwards compatible yet clean, and generates client/server code in a variety of languages.")]
pub struct Args {
    #[structopt(long = "migration-path", short = "d", default_value=DEFAULT_MIGRATION_DIR, env = "APIVOLVE_MIGRATION_PATH")]
    pub migration_dirs: Vec<String>,
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
