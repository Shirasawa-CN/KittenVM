use std::fmt::{Debug, Formatter};
#[derive(Clone)]
pub enum Type<T> {
    int(i128),
    nnint(usize),
    fract(f64),
    bool(bool),
    char(char),
    vector(Vec<T>),
    none,
}

impl<T: std::fmt::Debug> Debug for Type<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::int(i) => write!(f, "{}", i),
            Type::nnint(i) => write!(f, "{}", i),
            Type::fract(i) => write!(f, "{:.5}", i),
            Type::char(i) => write!(f, "'{}'", i),
            Type::bool(i) => write!(f, "{}", i),
            Type::vector(i) => write!(f, "{:#?}", i),
            Type::none => write!(f, "none"),
        }
    }
}
