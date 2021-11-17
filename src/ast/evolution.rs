use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::ast::object::ObjectOp;

lazy_static! {
    static ref PATH_RE: Regex = Regex::new(r"[a-zA-Z_][a-zA-Z0-9_]*(/[a-zA-Z_][a-zA-Z0-9_]*)*(.apiv)?").unwrap();
}

#[derive(Debug)]
pub struct EvolutionAst {
    pub depends: Vec<Dependency>,
    pub blocks: Vec<Block>,
}

impl EvolutionAst {
    pub fn new(version: String, depends: Vec<Dependency>, blocks: Vec<Block>) -> Self {
        EvolutionAst {
            depends,
            blocks,
        }
    }
}

#[derive(Debug)]
pub enum Block {
    Obj(ObjectOp),
}

#[derive(Debug)]
pub struct Dependency {
    path: Path,
    hash: Option<String>,
}

impl Dependency {
    pub fn fixed(path: Path, hash: &str) -> Self {
        Dependency {
            path,
            hash: Some(hash.to_owned()),
        }
    }

    pub fn dynamic(path: Path) -> Self {
        Dependency {
            path,
            hash: None,
        }
    }

    pub fn is_fixed(&self) -> bool {
        self.hash.is_some()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path {
    path: String,
}

impl Path {
    pub fn new(path: &str) -> Self {
        Path {
            path: path.to_owned(),
        }
    }

    pub fn is_valid(&self) -> bool {
        PATH_RE.is_match(&self.path)
    }
}
