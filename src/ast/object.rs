use crate::ast::Span;
use crate::ast::term::{Expression, Iden, Value};

#[derive(Debug)]
pub enum ObjectOp {
    Add(ObjectAdd),
    Change(ObjectChange),
    Delete(ObjectDelete),
}

#[derive(Debug)]
pub struct ObjectAdd {
    pub name: Iden,
    pub desc: Option<(Span, String)>,
    pub ops: Vec<FieldOp>,
}

impl ObjectAdd {
    pub fn minimal(name: Iden, ops: Vec<FieldOp>,) -> Self {
        ObjectAdd { name, desc: None, ops }
    }

    pub fn with_desc(name: Iden, desc: (Span, &str), ops: Vec<FieldOp>,) -> Self {
        ObjectAdd { name, desc: Some((desc.0, desc.1.to_owned())), ops }
    }
}

#[derive(Debug)]
pub struct ObjectChange {
    pub name: Iden,
    pub desc: Option<(Span, Vec<String>)>,
    pub ops: Vec<FieldOp>,
}

impl ObjectChange {
    pub fn new(name: Iden, ops: Vec<FieldOp>) -> Self {
        ObjectChange { name, desc: None, ops }
    }
}

#[derive(Debug)]
pub struct ObjectDelete {
    pub name: Iden
}

#[derive(Debug)]
pub enum FieldOp {
    Add(Iden, Vec<FieldProperty>),
    Change(Iden, Vec<FieldProperty>),
    Delete(Iden),
}

#[derive(Debug)]
pub enum FieldProperty {
    Type(Iden),
    Description(String, Span),
    Default(Expression),
}
