use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Iden {
    name: String,
}

impl Iden {
    pub fn new(name: String) -> Self {
        Iden {
            name,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path {
    path: PathBuf,
}

impl Path {
    pub fn new(path: String) -> Self {
        Path {
            path: PathBuf::from(path),
        }
    }
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

#[derive(Debug)]
pub struct ObjectField {
    name: Iden,
    value: Iden,
}

impl ObjectField {
    pub fn new(name: Iden, value: Iden) -> Self {
        ObjectField {
            name,
            value,
        }
    }
}

#[derive(Debug)]
pub struct Block {
    name: Iden,
    fields: Vec<ObjectField>
}

impl Block {
    pub fn new(name: Iden, fields: Vec<ObjectField>) -> Self {
        Block {
            name,
            fields,
        }
    }
}

#[derive(Debug)]
pub struct EvolutionAst {
    depends: Vec<Dependency>,
    blocks: Vec<Block>,
}

impl EvolutionAst {
    pub fn new(depends: Vec<Dependency>, blocks: Vec<Block>) -> Self {
        EvolutionAst {
            depends,
            blocks,
        }
    }
}
