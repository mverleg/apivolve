use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::ast::evolution::VersionBump::Patch;
use crate::ast::object::ObjectOp;
use crate::ast::Span;
use crate::Version;

lazy_static! {
    static ref PATH_RE: Regex =
        Regex::new(r"[a-zA-Z_][a-zA-Z0-9_]*(/[a-zA-Z_][a-zA-Z0-9_]*)*(.apiv)?").unwrap();
}

#[derive(Debug)]
pub struct EvolutionAst {
    pub bump: (Option<Span>, VersionBump),
    pub depends: Vec<Dependency>,
    pub blocks: Vec<Block>,
}

impl EvolutionAst {
    pub fn new(
        apivolve_version: String,
        bump: Option<(Span, VersionBump)>,
        depends: Vec<Dependency>,
        blocks: Vec<Block>,
    ) -> Self {
        EvolutionAst {
            bump: match bump {
                Some(bump) => (Some(bump.0), bump.1),
                None => (None, Patch),
            },
            depends,
            blocks,
        }
    }
}

#[derive(Debug)]
pub enum VersionBump {
    Patch,
    Minor,
    Major,
}

#[derive(Debug, Clone)]
pub enum Block {
    Obj(ObjectOp),
}

#[derive(Debug, Clone)]
pub struct Dependency {
    //TODO @mark: switch from path to just version, since names are predictable now?
    //TODO @mark: might be a problem when doing git merges though...
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
        Dependency { path, hash: None }
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
            path: path[1..path.len() - 1].to_owned(),
        }
    }

    pub fn is_valid(&self) -> bool {
        PATH_RE.is_match(&self.path)
    }
}
