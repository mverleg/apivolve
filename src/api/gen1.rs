//! The generating executable should emit [GenerateConfig] as json on stdout.
//! Then Apivolve CLI will send [GenerateChangesInput] in desired format on its stdin.

use ::std::path::PathBuf;

use ::lazy_static::lazy_static;
use ::regex::Regex;
use ::serde::Deserialize;
use ::serde::Serialize;
use ::which::which_re;

use crate::Version;

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

pub fn find_all_generators() -> Vec<(String, PathBuf)> {
    which_re(&*RE_GEN_NAME).unwrap()
        .inspect(|mtch| println!("{:?}", &mtch))
        .collect::<Vec<_>>();
    todo!()
}

pub fn find_target_generators(names: &[String]) -> Vec<(String, PathBuf)> {
    todo!()
}
