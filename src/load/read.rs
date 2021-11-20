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

lazy_static! {
    static ref VER_RE: Regex =
        Regex::new(r"v([0-9]+)\.([0-9]+)\.([0-9]+)(\.[a-zA-Z0-9_\-]+)?\.apiv").unwrap();
}

#[derive(Debug)]
pub struct Evolution {
    pub path: PathBuf,
    pub version: Version,
    pub depends: Vec<Dependency>,
    pub blocks: Vec<Block>,
}

impl Evolution {
    pub fn seal(&self, hasher: &mut impl Update) {
        //TODO @mark:
        hasher.update((self.depends.len() as u32).to_le_bytes());
        hasher.update((self.blocks.len() as u32).to_le_bytes());
    }
}

#[derive(Debug)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    desc: Option<String>,
}

impl Version {
    pub fn pure(&self) -> Version {
        Version {
            major: self.major,
            minor: self.minor,
            patch: self.patch,
            desc: None,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.desc {
            Some(desc) => write!(f, "{}.{}.{}.{}", self.major, self.minor, self.patch, desc),
            None => write!(f, "{}.{}.{}", self.major, self.minor, self.patch),
        }
    }
}

pub fn load_dirs(paths: Vec<PathBuf>) -> ApivResult<Vec<Evolution>> {
    let mut evolutions = vec![];
    for path in paths {
        evolutions.extend(load_dir(path)?);
    }
    Ok(evolutions)
}

fn load_dir(path: PathBuf) -> ApivResult<Vec<Evolution>> {
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
        version,
        depends: ast.depends,
        blocks: ast.blocks,
    })
}

fn extract_version(path: &Path) -> ApivResult<Version> {
    let name_os = path.file_name().ok_or_else(|| {
        format!(
            "Could not get basename from evolution path '{}'",
            path.to_string_lossy()
        )
    })?;
    let name = name_os.to_str().ok_or_else(|| {
        format!(
            "Filename '{}' does not seem to be UTF8-encoded",
            path.to_string_lossy()
        )
    })?;
    let groups = VER_RE.captures(name).ok_or_else(|| {
        format!(
            "Evolution filename '{}' should follow a strict naming convention - \
        'v1.2.3.apiv' or 'v1.2.3.description.apiv', starting with 'v', three-digit semver, \
        optional description and ending with extension '.apiv'",
            name
        )
    })?;
    let desc = groups.get(4).map(|m| m.as_str().to_owned());
    if let Some(desc) = desc {
        //TODO: should descriptions be allowed? it is very helpful, but increases the chance to have duplicate versions without conflicts
        return Err(format!(
            "Filename should be just a version of 3 numbers, not '{}' in '{}'",
            &desc, path.to_string_lossy()
        ));
    }
    Ok(Version {
        major: groups[1].parse().unwrap(),
        minor: groups[2].parse().unwrap(),
        patch: groups[3].parse().unwrap(),
        desc: desc,
    })
}
