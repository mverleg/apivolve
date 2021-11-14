use ::std::path::PathBuf;
use std::ffi::OsStr;

use crate::common::ApivResult;

#[derive(Debug)]
pub struct Evolution {
    path: PathBuf,
}

fn load_dirs(paths: Vec<PathBuf>) -> ApivResult<Vec<Evolution>> {
    let mut evolutions = vec![];
    for path in paths {
        evolutions.extend(load_dir(path)?);
    }
    Ok(evolutions)
}

fn load_dir(path: PathBuf) -> ApivResult<Vec<Evolution>> {
    if !path.exists() {
        return Err(format!("tried to load migrations from directory {} but it does not exist", path.to_string_lossy()))
    }
    if !path.is_dir() {
        return Err(format!("tried to load migrations from directory {} but it is not a directory", path.to_string_lossy()))
    }
    let mut evolutions = vec![];
    let dir = path.read_dir().map_err(|err| format!("failed to load migrations from directory {} because of a technical problem: {}", path.to_string_lossy(), err))?;
    for sub in dir {
        let file = sub.map_err(|err| format!("failed to read entry from directory {} because of a technical problem: {}", path.to_string_lossy(), err))?;
        if file.path().extension() != Some(OsStr::new("apiv")) {
            continue;
        }
        let evolution = load_file(file.path().to_path_buf());
        evolutions.push(evolution);
    }
    Ok(evolutions)
}

fn load_file(path: PathBuf) -> Evolution {
    unimplemented!();
}
