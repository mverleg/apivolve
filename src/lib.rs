use ::std::path::PathBuf;
use crate::load::read::load_dirs;

pub use crate::common::ApivResult;

mod common;

mod load;

pub fn apivolve_check(evolution_dirs: Vec<PathBuf>) -> ApivResult<()> {
    evolution_dirs;
    unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub fn apivolve_generate(evolution_dirs: Vec<PathBuf>) -> ApivResult<()> {
    evolution_dirs;
    unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub fn apivolve_list(evolution_dirs: Vec<PathBuf>) -> ApivResult<()> {
    let evolutions = load_dirs(evolution_dirs)?;
    unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub fn apivolve_next(evolution_dirs: Vec<PathBuf>) -> ApivResult<()> {
    evolution_dirs;
    unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!
}
