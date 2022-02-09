use ::structopt::StructOpt;

pub static DEFAULT_EVOLUTION_DIR: &str = "./apivolve";

#[derive(Debug, StructOpt)]
#[structopt(
    name = "apivolve",
    about = "API evolution tool, it helps keep your APIs backwards compatible yet clean, and generates client/server code in a variety of languages."
)]
pub struct Args {
    #[structopt(long = "evolution-path", short = "d", default_value=DEFAULT_EVOLUTION_DIR, env = "APIVOLVE_EVOLUTION_PATH")]
    pub evolution_dir: String,
    #[structopt(subcommand)]
    pub cmd: Cmd,
}

#[derive(Debug, StructOpt)]
pub enum Cmd {
    #[structopt(about = "List all the evolutions in valid resolution order")]
    List {
        #[structopt(long, about = "Output as json v1")]
        json1: bool,
    },
    #[structopt(about = "Check that the API can be upgraded by running all evolutions")]
    Check {},
    #[structopt(about = "Generate the API code for a specific target(s)")]
    Gen {
        #[structopt(
            subcommand,
            about = "Targets to generate code for; leave empty to show all implementations"
        )]
        targets: Option<Targets>,
    },
    #[structopt(about = "Create a new evolutions file at the head of the current chain")]
    New {},
    #[structopt(about = "Combine current pending changes into a release")]
    Release {
        // #[structopt(short, long, about = "Squash all the evolutions into one file per version")]
        // squash: bool,
    },
}

#[derive(Debug, StructOpt)]
pub enum Targets {
    #[structopt(external_subcommand)]
    Targets(Vec<String>),
}
