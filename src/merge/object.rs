
use crate::ast::term::Iden;


#[derive(Debug)]
pub struct State {
    pub objects: Vec<Object>
}

#[derive(Debug)]
pub struct Object {
    pub fields: Vec<Field>
}

#[derive(Debug)]
pub struct Field {
    pub name: Iden,
    pub typ: Iden,
}

