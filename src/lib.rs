#![allow(unused)] //TODO temporary

use ::std::collections::HashMap;
use ::std::hash::BuildHasherDefault;
use ::std::hash::Hasher;
use ::std::path::PathBuf;
use ::std::io::repeat;

use ::sha2::Digest;
use ::sha2::Sha256;

pub use crate::common::ApivResult;
use crate::load::read::load_dirs;
use crate::load::version::Version;

mod common;

mod ast;
mod load;
mod merge;

pub fn apivolve_check(_evolution_dirs: Vec<PathBuf>) -> ApivResult<()> {
    unimplemented!() //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub fn apivolve_generate(_evolution_dirs: Vec<PathBuf>) -> ApivResult<()> {
    unimplemented!() //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub fn apivolve_list(evolution_dirs: Vec<PathBuf>) -> ApivResult<()> {
    let evolutions = load_dirs(evolution_dirs)?;
    let mut prev_version = Version::new(0, 0, 0);
    if !evolutions.iter().next().map(|kv| kv.0 != &prev_version).unwrap_or(true) {
        println!("{}", prev_version);
    }
    for (version, evolutions) in evolutions {
        let mut hasher = Sha256::new();
        evolutions.seal(&mut hasher);
        let digest = format!("sha256:{}", base64::encode(hasher.finalize()));
        let depth = depth(&prev_version, &version) as usize;
        prev_version = version.clone();
        println!(
            "{}{}\t\"{}\"",
            "  ".repeat(depth),
            &version,
            digest,
        );
        for evolution in &evolutions {
            println!(
                "{}  \t\"{}\"",
                "  ".repeat(depth),
                evolution.path.to_string_lossy(),
            );
        }
    }
    Ok(())
}

pub fn depth(prev: &Version, cur: &Version) -> u8 {
    assert!(prev <= cur);
    if prev.major() < cur.major() {
        return 0;
    }
    if prev.minor() < cur.minor() {
        return 1;
    }
    2
}

pub fn apivolve_next(_evolution_dirs: Vec<PathBuf>) -> ApivResult<()> {
    unimplemented!() //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub fn apivolve_release(_evolution_dirs: Vec<PathBuf>) -> ApivResult<()> {
    unimplemented!() //TODO @mark: TEMPORARY! REMOVE THIS!
}
