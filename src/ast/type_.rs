#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Unit,
    Int,
    Double,
    Array(Box<Type>, usize),
}
