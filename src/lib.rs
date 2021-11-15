#![allow(unused)]  //TODO temporary

use ::std::path::PathBuf;
use crate::load::read::load_dirs;

pub use crate::common::ApivResult;

mod common;

mod load;

pub fn apivolve_check(_evolution_dirs: Vec<PathBuf>) -> ApivResult<()> {
    unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub fn apivolve_generate(_evolution_dirs: Vec<PathBuf>) -> ApivResult<()> {
    unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub fn apivolve_list(evolution_dirs: Vec<PathBuf>) -> ApivResult<()> {
    let _evolutions = load_dirs(evolution_dirs)?;
    unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub fn apivolve_next(_evolution_dirs: Vec<PathBuf>) -> ApivResult<()> {
    unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!
}
