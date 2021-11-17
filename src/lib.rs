#![allow(unused)]  //TODO temporary

use ::std::collections::HashMap;
use ::std::hash::BuildHasherDefault;
use ::std::hash::Hasher;
use ::std::path::PathBuf;

use ::twox_hash::XxHash64;

pub use crate::common::ApivResult;
use crate::load::read::load_dirs;

mod common;

mod load;
mod ast;

pub fn apivolve_check(_evolution_dirs: Vec<PathBuf>) -> ApivResult<()> {
    unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub fn apivolve_generate(_evolution_dirs: Vec<PathBuf>) -> ApivResult<()> {
    unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub fn apivolve_list(evolution_dirs: Vec<PathBuf>) -> ApivResult<()> {
    let evolutions = load_dirs(evolution_dirs)?;
    let mut hasher = XxHash64::default();
    evolutions[0].seal(&mut hasher);
    let digest = format!("xx64:{}", base64::encode(hasher.finish().to_le_bytes()));
    dbg!(digest);
    //dbg!(evolutions);  //TODO @mark: TEMPORARY! REMOVE THIS!
    unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub fn apivolve_next(_evolution_dirs: Vec<PathBuf>) -> ApivResult<()> {
    unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!
}
