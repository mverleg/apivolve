use std::path::PathBuf;
use std::ptr::hash;
use std::slice::Iter;

use sha2::digest::Update;

use crate::ast::evolution::{Block, Dependency};
use crate::Version;

/// Sorted and non-empty
#[derive(Debug)]
pub struct Evolutions {
    evolutions: Vec<Evolution>,
}

impl Evolutions {
    pub fn from(evolutions: Vec<Evolution>) -> Self {
        Evolutions { evolutions }
    }

    pub fn seal(&self, hasher: &mut impl Update) {
        for evolution in &self.evolutions {
            evolution.seal(hasher);
        }
    }
}

impl <'a> IntoIterator for &'a Evolutions {
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