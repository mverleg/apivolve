use std::path::PathBuf;
use std::slice::Iter;
use sha2::digest::Update;
use crate::ast::evolution::{Block, Dependency};
use crate::merge::linear::linearize;
use crate::Version;

/// Sorted and non-empty
#[derive(Debug)]
pub struct VersionEvolutions {
    version: Option<Version>,
    evolutions: Vec<Evolution>,
}

impl VersionEvolutions {
    pub fn from(version: Option<Version>, evolutions: Vec<Evolution>) -> Self {
        //TODO @mark: linearize
        linearize;
        VersionEvolutions { version, evolutions }
    }

    pub fn is_pending(&self) -> bool {
        self.version.is_none()
    }
}

impl <'a> IntoIterator for &'a VersionEvolutions {
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
        hasher.update((self.depends.len() as u32).to_le_bytes());
        hasher.update((self.blocks.len() as u32).to_le_bytes());
    }
}