use ::std::borrow::Borrow;
use ::std::collections::BTreeMap;
use ::std::ffi::OsStr;
use ::std::fmt;
use ::std::fs::read_to_string;
use ::std::fs::{DirEntry, ReadDir};
use ::std::hash::Hasher;
use ::std::io::Write;
use ::std::path::Path;
use ::std::path::PathBuf;

use ::log::debug;
use ::sha2::digest::Update;

use crate::ast::evolution::{Block, Dependency};
use crate::common::ApivResult;
use crate::load::compile::compile;
use crate::load::evolution::{Evolution, Evolutions, FullEvolution};
use crate::load::version::Version;

pub fn load_dir(apivdir_path: PathBuf) -> ApivResult<FullEvolution> {
    //TODO @mark: maybe scan with infinite recursion to warn about .apiv files being ignored
    //TODO @mark: more logging about things that are skipped
    let mut evolutions = BTreeMap::new();
    for entry in read_dir(&apivdir_path)? {
        let path = entry.path();
        if path.is_dir() {
            let version_dir_name = path
                .file_name()
                .ok_or_else(|| {
                    format!(
                        "could not get name for directory '{}'",
                        path.to_string_lossy()
                    )
                })?
                .to_str()
                .ok_or_else(|| {
                    format!(
                        "name for directory '{}' does not seem to be unicode",
                        path.to_string_lossy()
                    )
                })?;
            let version_evolutions = load_all_in_dir(path.as_path())?;
            if version_evolutions.is_empty() {
                debug!("skipping directory '{}' because it does not contain evolution files (non-recursive)", path.to_string_lossy());
                continue;
            }
            let version = Version::try_from(version_dir_name).map_err(|err| {
                format!(
                    "problem with evolution directory '{}': {}",
                    err,
                    path.to_string_lossy()
                )
            })?;
            evolutions.insert(version, Evolutions::from(version_evolutions));
        }
    }
    let pending_evolutions = load_all_in_dir(&apivdir_path)?;
    Ok(FullEvolution::new(
        evolutions,
        Evolutions::from_if_any(pending_evolutions),
    ))
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
    for released_entry in read_dir(path)? {
        let released_path = released_entry.path();
        if released_path.extension() != Some(OsStr::new("apiv")) {
            continue;
        }
        let evolution = load_file(released_path.to_path_buf())?;
        evolutions.push(evolution);
    }
    Ok(evolutions)
}

fn load_file(path: PathBuf) -> ApivResult<Evolution> {
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
