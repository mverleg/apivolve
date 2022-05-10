use ::ustr::Ustr;

use crate::ast::Span;
use crate::ast::term::{Expression, Iden, Value};

#[derive(Debug)]
pub struct ObjectEvolution {
    pub identifier: Iden,
    pub operation: ObjectOp,
}

impl ObjectEvolution {
    pub fn new(identifier: Iden, operation: ObjectOp) -> Self {
        ObjectEvolution {
            identifier,
            operation,
        }
    }
}

#[derive(Debug)]
pub enum ObjectOp {
    Add(ObjectAdd),
    Change(ObjectChange),
    Delete(ObjectDelete),
}

#[derive(Debug)]
pub struct ObjectAdd {
    pub desc: Option<(Span, String)>,
    pub operations: Vec<FieldEvolution>,
}

impl ObjectAdd {
    pub fn minimal(operations: Vec<FieldEvolution>) -> Self {
        ObjectAdd {
            desc: None,
            operations,
        }
    }

    pub fn with_desc(desc: (Span, &str), operations: Vec<FieldEvolution>) -> Self {
        ObjectAdd {
            desc: Some((desc.0, desc.1.to_owned())),
            operations,
        }
    }
}

#[derive(Debug)]
pub struct ObjectChange {
    pub desc: Option<(Span, String)>,
    pub operations: Vec<FieldEvolution>,
}

impl ObjectChange {
    pub fn minimal(operations: Vec<FieldEvolution>) -> Self {
        ObjectChange {
            desc: None,
            operations,
        }
    }

    pub fn with_desc(desc: (Span, &str), operations: Vec<FieldEvolution>) -> Self {
        ObjectChange {
            desc: Some((desc.0, desc.1.to_owned())),
            operations,
        }
    }
}

#[derive(Debug)]
pub struct ObjectDelete {}

#[derive(Debug)]
pub struct FieldEvolution {
    pub identifier: Iden,
    pub operation: FieldOp,
}

impl FieldEvolution {
    pub fn new(identifier: Iden, operation: FieldOp) -> Self {
        FieldEvolution {
            identifier,
            operation,
        }
    }
}

#[derive(Debug)]
pub enum FieldOp {
    Add(Vec<FieldProperty>),
    Change(Vec<FieldProperty>),
    Delete(),
}

#[derive(Debug)]
pub enum FieldProperty {
    Name(Iden),
    Type(Iden),
    Description(String, Span),
    Default(Expression),
}
