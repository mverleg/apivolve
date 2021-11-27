use ::std::borrow::Borrow;
use ::std::collections::BTreeMap;
use ::std::ffi::OsStr;
use ::std::fmt;
use ::std::fs::{DirEntry, ReadDir};
use ::std::fs::read_to_string;
use ::std::hash::Hasher;
use ::std::io::Write;
use ::std::path::Path;
use ::std::path::PathBuf;

use ::lazy_static::lazy_static;
use ::regex::Regex;
use ::sha2::digest::Update;
use ::log::debug;

use crate::ast::evolution::{Block, Dependency};
use crate::common::ApivResult;
use crate::load::compile::compile;
use crate::load::evolution::{Evolution, Evolutions, FullEvolution};
use crate::load::version::{extract_version, Version};

lazy_static! {
    static ref VERSION_RE: Regex = Regex::new(r"v([0-9]+)([0-9]+)([0-9]+)\.apiv").unwrap();
}

pub fn load_dir(apivdir_path: PathBuf) -> ApivResult<FullEvolution> {
    //TODO @mark: maybe scan with infinite recursion to warn about .apiv files being ignored
    let mut evolutions = BTreeMap::new();
    for top_entry in read_dir(&apivdir_path)? {
        let top_path = top_entry.path();
        if top_path.is_dir() {
            let version_dir_name = top_path.file_name()
                .ok_or_else(|| format!("could not get name for directory '{}'", top_path.to_string_lossy()))?
                .to_str()
                .ok_or_else(|| format!("name for directory '{}' does not seem to be unicode", top_path.to_string_lossy()))?;
            let version_evolutions = load_all_in_dir(top_path.as_path())?;
            if version_evolutions.is_empty() {
                debug!("skipping directory '{}' because it does not contain evolution files (non-recursive)", top_path.to_string_lossy());
                continue;
            }
            let groups = VERSION_RE.captures(version_dir_name).ok_or_else(|| {
                format!("evolution directory '{}' should follow a strict naming convention - 'v1.2.3', starting with 'v', three-digit semver, no description or postfix or extension", top_path.to_string_lossy())
            })?;
            //groups.get(1).unwrap().parse().unwrap()
            evolutions.insert(version, Evolutions::from(version_evolutions))
        }
    }
    let pending_evolutions = load_all_in_dir(&apivdir_path)?;
    Ok(FullEvolution::new(evolutions, Evolutions::from_if_any(pending_evolutions)))
}

fn read_dir(path: &Path) -> ApivResult<Vec<DirEntry>> {
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
    let dir = path.read_dir().map_err(|err| {
        format!(
            "failed to load migrations from directory '{}' because of a technical problem: {}",
            path.to_string_lossy(),
            err
        )
    })?;
    dir.into_iter()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| {
            format!(
                "failed to read entry from directory '{}' because of a technical problem: {}",
                path.to_string_lossy(),
                err
            )
        })
}

/// Load all .apiv files in directory, ignore everything else.
fn load_all_in_dir(path: &Path) -> ApivResult<Vec<Evolution>> {
    let mut evolutions = vec![];
    for released_entry in read_dir(top_path.as_path())? {
        let released_path = released_entry.path();
        if released_path.extension() != Some(OsStr::new("apiv")) {
            continue
        }
        let evolution = load_file(released_path.to_path_buf())?;
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
