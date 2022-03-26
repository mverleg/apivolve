use ::ustr::Ustr;
use ::ustr::UstrMap;

use crate::ast::term::{Iden, Value};

#[derive(Debug)]
pub struct State {
    pub objects: UstrMap<Object>,
}

/// Qualified path
pub type ObjectId = Ustr;

#[derive(Debug)]
pub struct Object {
    pub id: ObjectId,
    pub description: String,
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Field {
    pub name: Iden,
    pub typ: Iden,
    pub description: String,
    pub default: Value,
}
