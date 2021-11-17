use crate::ast::Span;
use crate::ast::term::{Expression, Iden, Value};

#[derive(Debug)]
pub enum ObjectOp {
    Add(Iden, Vec<FieldOp>),
    Change(Iden, Vec<FieldOp>),
    Delete(Iden),
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
