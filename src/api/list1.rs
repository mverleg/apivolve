use ::std::fmt;
use ::std::fmt::Formatter;
use ::std::path::Path;
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

impl Listing {
    pub fn versions(&self) -> &[VersionListing] {
        &self.versions
    }

    pub fn pending(&self) -> &[EvolutionListing] {
        &self.pending
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionListing {
    version: Version,
    hash: String,
    depth: u8,
    evolutions: Vec<EvolutionListing>,
}

impl VersionListing {
    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }

    pub fn depth(&self) -> u8 {
        self.depth
    }

    pub fn evolutions(&self) -> &[EvolutionListing] {
        &self.evolutions
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvolutionListing {
    path: PathBuf,
}

impl EvolutionListing {
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
}

impl fmt::Display for Listing {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for version in self.versions() {
            println!("{}{} \"{}\"", "  ".repeat(version.depth() as usize), &version.version(), version.hash());
            print_evolutions(version.evolutions(), version.depth())
        }
        match self.pending() {
            &[] => println!("pending: none"),
            pend => {
                println!("pending");
                print_evolutions(pend, 0);
            }
        }
        Ok(())
    }
}

pub async fn apivolve_list(evolution_dir: PathBuf) -> ApivResult<Listing> {
    let evolutions = load_dir(evolution_dir)?;
    let mut prev_version = Version::new(0, 0, 0);
    let mut versions = vec![];
    for (version, evolutions) in evolutions.released() {
        let hash = evolutions.seal();
        let depth = depth(&prev_version, version);
        let mut evolution_listings = vec![];
        for evolution in evolutions {
            evolution_listings.push(EvolutionListing {
                path: evolution.path.to_path_buf()
            })
        }
        versions.push(VersionListing {
            version: version.clone(),
            hash,
            depth,
            evolutions: evolution_listings,
        });
        prev_version = version.clone();
    }
    let mut pending = vec![];
    if let Some(evolutions) = evolutions.pending() {
        for evolution in evolutions {
            pending.push(EvolutionListing {
                path: evolution.path.to_path_buf()
            })
        }
    }
    Ok(Listing {
        versions,
        pending,
    })
}

fn print_evolutions(evolutions: &[EvolutionListing], depth: u8) {
    for evolution in evolutions {
        println!("{}- \"{}\"", "  ".repeat(depth as usize), evolution.path().to_string_lossy());
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
