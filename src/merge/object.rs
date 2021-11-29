use crate::ast::term::{Iden, Value};

#[derive(Debug)]
pub struct State {
    pub objects: Vec<Object>,
}

#[derive(Debug)]
pub struct Object {
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
