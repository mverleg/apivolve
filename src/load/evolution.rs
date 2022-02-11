use ::std::collections::BTreeMap;
use ::std::path::PathBuf;
use ::std::ptr::hash;
use ::std::slice::Iter;

use ::sha2::Digest;
use ::sha2::digest::Update;
use sha2::Sha256;

use crate::ast::evolution::{Block, Dependency};
use crate::Version;

#[derive(Debug)]
pub struct FullEvolution {
    released: BTreeMap<Version, Evolutions>,
    pending: Option<Evolutions>,
}

impl FullEvolution {
    pub fn new(released: BTreeMap<Version, Evolutions>, pending: Option<Evolutions>) -> Self {
        FullEvolution { released, pending }
    }

    pub fn released(&self) -> &BTreeMap<Version, Evolutions> {
        &self.released
    }

    pub fn pending(&self) -> &Option<Evolutions> {
        &self.pending
    }
}

/// Sorted and non-empty
#[derive(Debug)]
pub struct Evolutions {
    evolutions: Vec<Evolution>,
}

impl Evolutions {
    pub fn from(evolutions: Vec<Evolution>) -> Self {
        assert!(!evolutions.is_empty());
        Evolutions { evolutions }
    }

    pub fn from_if_any(evolutions: Vec<Evolution>) -> Option<Self> {
        if evolutions.is_empty() {
            return None;
        }
        Some(Self::from(evolutions))
    }

    pub fn evolution(&self) -> &[Evolution] {
        &self.evolutions
    }

    pub fn seal_with(&self, hasher: &mut impl Update) {
        for evolution in &self.evolutions {
            evolution.seal(hasher);
        }
    }

    pub fn seal(&self) -> String {
        let mut hasher = Sha256::new();
        self.seal_with(&mut hasher);
        format!("sha256:{}", base64::encode(hasher.finalize()))
    }
}

impl<'a> IntoIterator for &'a Evolutions {
    type Item = &'a Evolution;
    type IntoIter = Iter<'a, Evolution>;

    fn into_iter(self) -> Self::IntoIter {
        self.evolutions.iter()
    }
}

#[derive(Debug)]
pub struct Evolution {
    pub path: PathBuf,
    pub depends: Vec<Dependency>,
    pub blocks: Vec<Block>,
}

impl Evolution {
    pub fn seal(&self, hasher: &mut impl Update) {
        //TODO @mark:
        hasher.update((self.depends.len() as u32).to_le_bytes().as_slice());
        hasher.update((self.blocks.len() as u32).to_le_bytes().as_slice());
    }
}
