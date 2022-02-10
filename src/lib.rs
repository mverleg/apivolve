#![allow(unused)] //TODO temporary

use ::std::collections::HashMap;
use ::std::hash::BuildHasherDefault;
use ::std::hash::Hasher;
use ::std::io::repeat;
use ::std::path::PathBuf;

use ::sha2::Digest;
use ::sha2::Sha256;

pub use crate::common::ApivResult;
use crate::load::evolution::{Evolutions, FullEvolution};
use crate::load::read::load_dir;
use crate::load::version::Version;

mod common;

mod ast;
mod load;
mod merge;

pub async fn apivolve_check(evolution_dir: PathBuf) -> ApivResult<()> {
    unimplemented!() //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub async fn apivolve_generate(evolution_dir: PathBuf, targets: &[String]) -> ApivResult<()> {
    assert!(!targets.is_empty(), "need at least one target");
    unimplemented!() //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub async fn apivolve_list(evolution_dir: PathBuf, json: bool) -> ApivResult<()> {
    let evolutions = load_dir(evolution_dir)?;
    let mut prev_version = Version::new(0, 0, 0);
    if !evolutions
        .released()
        .iter()
        .next()
        .map(|kv| kv.0 != &prev_version)
        .unwrap_or(true)
    {
        println!("{}", prev_version);
    }
    if json {
        list_json(&evolutions, prev_version)
    } else {
        list_text(&evolutions, prev_version)
    }
    Ok(())
}

fn list_text(evolutions: &FullEvolution, mut prev_version: Version) {
    for (version, evolutions) in evolutions.released() {
        let mut hasher = Sha256::new();
        evolutions.seal(&mut hasher);
        let digest = format!("sha256:{}", base64::encode(hasher.finalize()));
        let depth = depth(&prev_version, version) as usize;
        prev_version = version.clone();
        println!("{}{}\t\"{}\"", "  ".repeat(depth), &version, digest,);
        print_evolutions(evolutions, depth)
    }
    if let Some(pending) = evolutions.pending() {
        println!("pending");
        print_evolutions(pending, 0);
    } else {
        println!("pending: none");
    }
}

fn list_json(evolutions: &FullEvolution, mut prev_version: Version) {
    println!("{{");
    for (version, evolutions) in evolutions.released() {
        let mut hasher = Sha256::new();
        evolutions.seal(&mut hasher);
        let digest = format!("sha256:{}", base64::encode(hasher.finalize()));
        let depth = depth(&prev_version, version) as usize + 2;
        prev_version = version.clone();
        println!("{}\"{}\": {{\"hash\": \"{}\"}}", "  ".repeat(depth), &version, digest,);
        print_json_evolutions(evolutions, depth)
    }
    print!("\"pending\": ");
    if let Some(pending) = evolutions.pending() {
        print_json_evolutions(pending, 0);
    } else {
        println!("[]");
    }
    println!("}}");
}

fn print_evolutions(evolutions: &Evolutions, depth: usize) {
    for evolution in evolutions {
        println!(
            "{}  \t\"{}\"",
            "  ".repeat(depth),
            evolution.path.to_string_lossy(),
        );
    }
}

fn print_json_evolutions(evolutions: &Evolutions, depth: usize) {
    print!("[\n");
    for evolution in evolutions {
        println!(
            "{}  \t\"{}\"",
            "  ".repeat(depth + 2),
            evolution.path.to_string_lossy(),
        );
    }
    println!("{}  ]", "  ".repeat(depth));
}

fn depth(prev: &Version, cur: &Version) -> u8 {
    assert!(prev <= cur);
    if prev.major() < cur.major() {
        return 0;
    }
    if prev.minor() < cur.minor() {
        return 1;
    }
    2
}

pub async fn apivolve_next(evolution_dir: PathBuf) -> ApivResult<()> {
    unimplemented!() //TODO @mark: TEMPORARY! REMOVE THIS!
}

pub async fn apivolve_release(evolution_dir: PathBuf) -> ApivResult<()> {
    unimplemented!() //TODO @mark: TEMPORARY! REMOVE THIS!
}
