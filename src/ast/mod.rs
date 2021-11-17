
pub mod object;
pub mod term;
pub mod evolution;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Span {
    left: usize,
    right: usize,
}

pub fn span(left: usize, right: usize) -> Span {
    Span { left, right }
}
