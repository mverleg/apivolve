use std::path::PathBuf;

use sha2::Sha256;

use crate::{ApivResult, Evolutions, FullEvolution, load_dir, Version};

#[derive(Debug, new, getter)]
pub struct VersionListing1 {
    version: Option<Version>,
    hash: String,
    evolutions: Vec<EvolutionListing1>,
    depth: u8,
}

#[derive(Debug)]
struct EvolutionListing1 {

}

pub async fn apivolve_list1(evolution_dir: PathBuf, json: bool) -> ApivResult<Vec<VersionListing1>> {
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
