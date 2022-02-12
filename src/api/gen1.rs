//! The generating executable should emit [GenerateConfig] as json on stdout.
//! Then Apivolve CLI will send [GenerateChangesInput] in desired format on its stdin.

use ::std::borrow::Cow;
use ::std::env;
use ::std::fmt;
use ::std::fmt::Formatter;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::vec::IntoIter;

use ::lazy_static::lazy_static;
use ::log::info;
use ::regex::Regex;
use ::serde::Deserialize;
use ::serde::Serialize;
use ::which::which;
use ::which::which_re;

use crate::{ApivResult, Version};

const GEN_NAME_PREFIX: &str = "apivolve-gen-";

lazy_static! {
    static ref GEN_NAME_RE: Regex = Regex::new(&format!("^{}.*", GEN_NAME_PREFIX)).unwrap();
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
    pub fn from_path(path: PathBuf) -> Self {
        let full_name = path.file_name().expect("no filename").to_str().expect("filename is not unicode");
        assert!(full_name.starts_with("apivolve-gen-"));
        let name = full_name[GEN_NAME_PREFIX.len()..].to_owned();
        Generator { name, path }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Generators {
    generators: Vec<Generator>,
}

pub async fn apivolve_list_generators() -> ApivResult<Generators> {
    find_all_generators()
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
        for gen in &self.generators {
            writeln!(f, "{} (@ {})", gen.name, gen.path.to_string_lossy());
        }
        Ok(())
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
    assert!(!targets.is_empty());
    if targets.is_empty() {
        return Err("Need at least one target to generate".to_owned())
    }
    for generator in find_target_generators(&targets).into_iter() {
        info!("starting generator {} (at {})", generator.name(), generator.path().to_string_lossy());
    }
    todo!() //TODO @mark: TEMPORARY! REMOVE THIS!
}

fn find_all_generators() -> ApivResult<Generators> {
    let generators = which_re(&*GEN_NAME_RE).unwrap()
        .map(Generator::from_path)
        .collect::<Vec<_>>();
    if generators.is_empty() {
        return Err("no generators found on $PATH".to_owned());
    }
    Ok(Generators {
        generators,
    })
}

fn find_target_generators(names: &[String]) -> ApivResult<Generators> {
    Ok(Generators {
        generators: names.iter()
            .map(|name| {
                let gen_name = format!("{}{}", GEN_NAME_PREFIX, &name);
                match which(gen_name) {
                    Ok(path) => Ok(Generator { name: name.to_owned(), path }),
                    Err(_) => Err(format!("failed to find executable {}", gen_name)),
                }
            })
            .collect::<ApivResult<Vec<_>>>()?
    })
}
