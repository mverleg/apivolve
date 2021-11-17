use ::std::fmt;

pub mod object;
pub mod term;
pub mod evolution;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Span {
    left: usize,
    right: usize,
}

pub fn span(left: usize, right: usize) -> Span {
    Span { left, right }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}-{}]", self.left, self.right)
    }
}
