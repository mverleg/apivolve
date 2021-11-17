#![allow(unused)]  //TODO temporary

use ::std::path::PathBuf;

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
    dbg!(evolutions);  //TODO @mark: TEMPORARY! REMOVE THIS!
    unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub fn apivolve_next(_evolution_dirs: Vec<PathBuf>) -> ApivResult<()> {
    unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!
}
