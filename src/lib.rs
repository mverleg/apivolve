#![allow(unused)] //TODO temporary

use ::std::collections::HashMap;
use ::std::hash::BuildHasherDefault;
use ::std::hash::Hasher;
use ::std::io::repeat;
use ::std::path::PathBuf;

use ::log::info;
use ::sha2::Digest;
use ::sha2::Sha256;

use crate::api::gen1::find_target_generators;
pub use crate::api::list1;
pub use crate::common::ApivResult;
use crate::load::evolution::{Evolutions, FullEvolution};
use crate::load::read::load_dir;
use crate::load::version::Version;

mod common;

mod ast;
mod load;
mod merge;
mod api;

pub async fn apivolve_check(evolution_dir: PathBuf) -> ApivResult<()> {
    todo!() //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub async fn apivolve_generate(evolution_dir: PathBuf, targets: &[String]) -> ApivResult<()> {
    if targets.is_empty() {
        return Err("Need at least one target to generate".to_owned())
    }
    for (target, generator) in find_target_generators(targets) {
        info!("starting generator {} (at {})", target, generator.to_string_lossy());
    }
    todo!() //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub async fn apivolve_next(evolution_dir: PathBuf) -> ApivResult<()> {
    todo!() //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub async fn apivolve_release(evolution_dir: PathBuf) -> ApivResult<()> {
    todo!() //TODO @mark: TEMPORARY! REMOVE THIS!
}
