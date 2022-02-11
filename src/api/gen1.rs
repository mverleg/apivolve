//! The generating executable should emit [GenerateConfig] as json on stdout.
//! Then Apivolve CLI will send [GenerateChangesInput] in desired format on its stdin.

use ::std::fmt;
use ::std::fmt::Formatter;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::vec::IntoIter;
use std::borrow::Cow;

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
    name: String,
    path: PathBuf,
}

impl Generator {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Generators {
    generators: Vec<Generator>,
}

pub async fn apivolve_list_generators() -> ApivResult<Generators> {
    Ok(find_all_generators())
}

impl IntoIterator for Generators {
    type Item = Generator;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.generators.into_iter()
    }
}

impl fmt::Display for Generators {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateResult {
    generators: Vec<Generator>,
}

impl fmt::Display for GenerateResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

pub async fn apivolve_generate(evolution_dir: PathBuf, targets: &[String]) -> ApivResult<GenerateResult> {
    assert!(targets.is_empty());
    if targets.is_empty() {
        return Err("Need at least one target to generate".to_owned())
    }
    for generator in find_target_generators(&targets).into_iter() {
        info!("starting generator {} (at {})", generator.name(), generator.path().to_string_lossy());
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
