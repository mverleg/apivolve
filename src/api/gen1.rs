//! The generating executable should emit [GenerateConfig] as json on stdout.
//! Then Apivolve CLI will send [GenerateChangesInput] in desired format on its stdin.

use ::std::path::PathBuf;

use ::lazy_static::lazy_static;
use ::log::info;
use ::regex::Regex;
use ::serde::Deserialize;
use ::serde::Serialize;
use ::which::which_re;

use crate::{ApivResult, Version};

lazy_static! {
    static ref RE_GEN_NAME: Regex = Regex::new("^apivolve-gen-.*").unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GenerateInputFormat {
    Json,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateConfig {
    apivolve_version: Version,
    format: GenerateInputFormat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateChangesInput {

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Generator {

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Generators {
    generators: Vec<Generator>,
}

pub async fn apivolve_list_generators() -> ApivResult<Generators> {
    find_all_generators()
}

pub async fn apivolve_generate(evolution_dir: PathBuf, targets: Vec<String>) -> ApivResult<()> {
    assert!(targets.is_empty());
    if targets.is_empty() {
        return Err("Need at least one target to generate".to_owned())
    }
    for (target, generator) in find_target_generators(targets) {
        info!("starting generator {} (at {})", target, generator.to_string_lossy());
    }
    todo!() //TODO @mark: TEMPORARY! REMOVE THIS!
}

fn find_all_generators() -> Generators {
    which_re(&*RE_GEN_NAME).unwrap()
        .inspect(|mtch| println!("{:?}", &mtch))
        .collect::<Vec<_>>();
    todo!()
}

fn find_target_generators(names: &[String]) -> Generators {
    todo!()
}
