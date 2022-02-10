use ::std::fmt;
use ::std::fmt::Formatter;
use ::std::path::PathBuf;

use ::serde::Deserialize;
use ::serde::Serialize;
use ::sha2::Digest;
use ::sha2::Sha256;

use crate::{ApivResult, Evolutions, FullEvolution, load_dir, Version};

#[derive(Debug, Serialize, Deserialize)]
pub struct Listing {
    versions: Vec<VersionListing>,
    pending: Vec<EvolutionListing>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionListing {
    version: Version,
    hash: String,
    depth: u8,
    evolutions: Vec<EvolutionListing>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EvolutionListing {
    path: PathBuf,
}

impl fmt::Display for Listing {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

pub async fn apivolve_list(evolution_dir: PathBuf) -> ApivResult<Listing> {
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
    Ok(vec![])
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

fn print_evolutions(evolutions: &Evolutions, depth: usize) {
    for evolution in evolutions {
        println!(
            "{}  \t\"{}\"",
            "  ".repeat(depth),
            evolution.path.to_string_lossy(),
        );
    }
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
