use crate::ast::term::{Expression, Iden, Value};
use crate::ast::Span;

#[derive(Debug, Clone)]
pub enum ObjectOp {
    Add(ObjectAdd),
    Change(ObjectChange),
    Delete(ObjectDelete),
}

#[derive(Debug, Clone)]
pub struct ObjectAdd {
    pub name: Iden,
    pub desc: Option<(Span, String)>,
    pub ops: Vec<FieldOp>,
}

impl ObjectAdd {
    pub fn minimal(name: Iden, ops: Vec<FieldOp>) -> Self {
        ObjectAdd {
            name,
            desc: None,
            ops,
        }
    }

    pub fn with_desc(name: Iden, desc: (Span, &str), ops: Vec<FieldOp>) -> Self {
        ObjectAdd {
            name,
            desc: Some((desc.0, desc.1.to_owned())),
            ops,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ObjectChange {
    pub name: Iden,
    pub desc: Option<(Span, String)>,
    pub ops: Vec<FieldOp>,
}

impl ObjectChange {
    pub fn minimal(name: Iden, ops: Vec<FieldOp>) -> Self {
        ObjectChange {
            name,
            desc: None,
            ops,
        }
    }

    pub fn with_desc(name: Iden, desc: (Span, &str), ops: Vec<FieldOp>) -> Self {
        ObjectChange {
            name,
            desc: Some((desc.0, desc.1.to_owned())),
            ops,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ObjectDelete {
    pub name: Iden,
}

#[derive(Debug, Clone)]
pub enum FieldOp {
    Add(Iden, Vec<FieldProperty>),
    Change(Iden, Vec<FieldProperty>),
    Delete(Iden),
}

#[derive(Debug, Clone)]
pub enum FieldProperty {
    Name(Iden),
    Type(Iden),
    Description(String, Span),
    Default(Expression),
}
