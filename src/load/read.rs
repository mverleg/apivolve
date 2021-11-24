use ::std::borrow::Borrow;
use ::std::ffi::OsStr;
use ::std::fmt;
use ::std::fs::read_to_string;
use ::std::hash::Hasher;
use ::std::io::Write;
use ::std::path::Path;
use ::std::path::PathBuf;

use ::lazy_static::lazy_static;
use ::regex::Regex;
use ::sha2::digest::Update;

use crate::ast::evolution::{Block, Dependency};
use crate::common::ApivResult;
use crate::load::compile::compile;
use crate::load::version::{extract_version, Version};

pub fn load_dirs(paths: Vec<PathBuf>) -> ApivResult<SortedMap<Version, Evolutions>> {
    let mut evolutions = vec![];
    for path in paths {
        evolutions.extend(load_dir(path)?);
    }
    Ok(Evolutions::from(evolutions))
}

fn load_dir(path: PathBuf) -> ApivResult<Evolutions> {
    if !path.exists() {
        return Err(format!(
            "tried to load migrations from directory '{}' but it does not exist",
            path.to_string_lossy()
        ));
    }
    if !path.is_dir() {
        return Err(format!(
            "tried to load migrations from directory '{}' but it is not a directory",
            path.to_string_lossy()
        ));
    }
    let mut evolutions = vec![];
    let dir = path.read_dir().map_err(|err| {
        format!(
            "failed to load migrations from directory '{}' because of a technical problem: {}",
            path.to_string_lossy(),
            err
        )
    })?;
    for sub in dir {
        let file = sub.map_err(|err| {
            format!(
                "failed to read entry from directory '{}' because of a technical problem: {}",
                path.to_string_lossy(),
                err
            )
        })?;
        if file.path().extension() != Some(OsStr::new("apiv")) {
            continue;
        }
        let evolution = load_file(file.path().to_path_buf())?;
        evolutions.push(evolution);
    }
    Ok(evolutions)
}

fn load_file(path: PathBuf) -> ApivResult<Evolution> {
    let version = extract_version(&path)?;
    let code = read_to_string(&path).map_err(|err| {
        format!(
            "failed to read migration file '{}' because ofa technical problem: {}",
            path.to_string_lossy(),
            err
        )
    })?;
    let ast = compile(path.to_string_lossy().as_ref(), &code)?;
    Ok(Evolution {
        path,
        depends: ast.depends,
        blocks: ast.blocks,
    })
}
