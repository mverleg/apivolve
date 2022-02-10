#![allow(unused)] //TODO temporary

use ::std::collections::HashMap;
use ::std::hash::BuildHasherDefault;
use ::std::hash::Hasher;
use ::std::io::repeat;
use ::std::path::PathBuf;

use ::sha2::Digest;
use ::sha2::Sha256;

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
    unimplemented!() //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub async fn apivolve_generate(evolution_dir: PathBuf, targets: &[String]) -> ApivResult<()> {
    assert!(!targets.is_empty(), "need at least one target");
    unimplemented!() //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub async fn apivolve_next(evolution_dir: PathBuf) -> ApivResult<()> {
    unimplemented!() //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub async fn apivolve_release(evolution_dir: PathBuf) -> ApivResult<()> {
    unimplemented!() //TODO @mark: TEMPORARY! REMOVE THIS!
}
