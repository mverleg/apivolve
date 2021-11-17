use crate::ast::Span;
use crate::ast::term::{Iden, Value};

#[derive(Debug)]
pub enum ObjectOp {
    Add(Iden, Vec<FieldOp>, Span),
    Change(Iden, Vec<FieldOp>, Span),
    Delete(Iden, Span),
}

#[derive(Debug)]
pub enum FieldOp {
    Add(Iden, Vec<FieldProperty>, Span),
    Change(Iden, Vec<FieldProperty>, Span),
    Delete(Iden, Span),
}

#[derive(Debug)]
pub enum FieldProperty {
    Type(Iden, Span),
    Description(String, Span),
    Default(Option<Value>, Span),
}
