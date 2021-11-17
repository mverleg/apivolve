use crate::ast::{Span, span};

#[derive(Debug)]
pub enum Value {
    Str(String, Span),
    Int(i64, Span),
    Real(f64, Span),
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Iden {
    name: String,
    span: Span,
}

impl Iden {
    pub fn new(name: String, left: usize, right: usize) -> Self {
        Iden {
            name,
            span: span(left, right),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Val(Value),
    Ref(Iden),
    None,
    Disabled,
}
