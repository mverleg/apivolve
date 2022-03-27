use crate::ast::{span, Span};
use ::std::fmt;

#[derive(Debug)]
pub enum Value {
    Str(String, Span),
    Int(i64, Span),
    Real(f64, Span),
    None(Span),
}

impl Value {
    pub fn str(quoted_text: &str, span: Span) -> Value {
        Value::Str(quoted_text[1..quoted_text.len() - 1].to_owned(), span)
    }

    pub fn int(nr: &str, span: Span) -> Value {
        Value::Int(nr.parse::<i64>().unwrap(), span)
    }

    pub fn real(nr: &str, span: Span) -> Value {
        Value::Real(nr.parse::<f64>().unwrap(), span)
    }
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

    pub fn new_backticked(name: String, left: usize, right: usize) -> Self {
        assert!(name.starts_with('`'));
        assert!(name.ends_with('`'));
        Iden {
            name: name[1..name.len() - 1].to_owned(),
            span: span(left, right),
        }
    }
}

impl fmt::Display for Iden {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug)]
pub enum Expression {
    Val(Value),
    Ref(Iden),
    None,
    Disabled,
}
