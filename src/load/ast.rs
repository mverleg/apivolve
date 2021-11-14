
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
pub struct Evolution {
    blocks: Vec<Block>,
}

impl Evolution {
    pub fn new(blocks: Vec<Block>) -> Self {
        Evolution {
            blocks,
        }
    }
}
