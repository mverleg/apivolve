#![allow(unused)]  //TODO temporary

use ::std::collections::HashMap;
use ::std::hash::BuildHasherDefault;
use ::std::hash::Hasher;
use ::std::path::PathBuf;

use ::sha2::Digest;
use ::sha2::Sha256;

pub use crate::common::ApivResult;
use crate::load::read::load_dirs;
use crate::merge::linear::linearize;

mod common;

mod load;
mod ast;
mod merge;


pub fn apivolve_check(_evolution_dirs: Vec<PathBuf>) -> ApivResult<()> {
    unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub fn apivolve_generate(_evolution_dirs: Vec<PathBuf>) -> ApivResult<()> {
    unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub fn apivolve_list(evolution_dirs: Vec<PathBuf>) -> ApivResult<()> {
    let evolutions = load_dirs(evolution_dirs)?;
    //let evolutions = linearize(evolutions);
    let mut hasher = Sha256::new();
    for evolution in evolutions {
        evolution.seal(&mut hasher);
        let digest = format!("xx64:{}", base64::encode(hasher.finish().to_le_bytes()));
        println!("{}\t{}", evolution.path.to_string_lossy(), digest);
    }
    unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub fn apivolve_next(_evolution_dirs: Vec<PathBuf>) -> ApivResult<()> {
    unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!
}
