use ::std::borrow::Borrow;
use ::std::collections::BTreeMap;
use ::std::ffi::OsStr;
use ::std::fmt;
use ::std::fs::read_to_string;
use ::std::hash::Hasher;
use ::std::io::Write;
use ::std::path::Path;
use ::std::path::PathBuf;
use std::fs::{DirEntry, ReadDir};

use ::lazy_static::lazy_static;
use ::regex::Regex;
use ::sha2::digest::Update;

use crate::ast::evolution::{Block, Dependency};
use crate::common::ApivResult;
use crate::load::compile::compile;
use crate::load::evolution::{Evolution, Evolutions};
use crate::load::version::{extract_version, Version};

lazy_static! {
    static ref PARTIAL_VERSION_RE: Regex =
        Regex::new(r"v([0-9]+)(\.([0-9]+)(\.([0-9]+))?)?\.apiv").unwrap();
}

pub fn load_dir(path: PathBuf) -> ApivResult<(Evolutions, BTreeMap<Version, Evolutions>)> {
    let mut evolutions = vec![];
    for entry in read_dir(&path)? {
        if entry.path().is_dir() {
            //TODO @mark: should I validate all dirs, or just the ones that have .apiv files?
            let groups = PARTIAL_VERSION_RE.captures(name).ok_or_else(|| {
                format!(
                    "Evolution directory '{}' should follow a strict naming convention - \
            'v1.2.3.apiv' or 'v1.2.3.description.apiv', starting with 'v', three-digit semver, \
            optional description and ending with extension '.apiv'",
                    name
                )
            })?;
            groups.get(1).unwrap().parse().unwrap()
            //TODO @mark: handle dir
        } else if file.path().is_file() {
            if file.path().extension() != Some(OsStr::new("apiv")) {
                continue
            }
            let evolution = load_file(file.path().to_path_buf())?;
            evolutions.push(evolution);
        }
    }
    Ok(evolutions)
}

fn read_dir(path: &PathBuf) -> ApivResult<Vec<DirEntry>> {
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
