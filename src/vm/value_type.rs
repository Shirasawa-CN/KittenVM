use std::{
    fmt::{Debug, Formatter},
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Shl, Shr, Sub},
};
#[derive(Clone, Copy)]
pub enum Type {
    int(i128),
    nnint(usize),
    fract(f64),
    bool(bool),
    char(char),
    none,
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::int(i) => write!(f, "{}", i),
            Type::nnint(i) => write!(f, "{}", i),
            Type::fract(i) => write!(f, "{:.5}", i),
            Type::char(i) => write!(f, "'{}'", i),
            Type::bool(i) => write!(f, "{}", i),
            Type::none => write!(f, "none"),
        }
    }
}

impl core::str::FromStr for Type {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match (
            s.parse::<i128>(),
            s.parse::<f64>(),
            s.parse::<usize>(),
            s.parse::<bool>(),
            s.parse::<char>(),
        ) {
            (Ok(i), _, _, _, _) => Ok(Type::int(i)),
            (Err(_), Ok(i), _, _, _) => Ok(Type::fract(i)),
            (Err(_), Err(_), Ok(i), _, _) => Ok(Type::nnint(i)),
            (Err(_), Err(_), Err(_), Ok(i), _) => Ok(Type::bool(i)),
            (Err(_), Err(_), Err(_), Err(_), Ok(i)) => Ok(Type::char(i)),
            (Err(_), Err(_), Err(_), Err(_), Err(_)) => Err("neither parser worked"),
        }
    }
}

impl Add for Type {
    type Output = Type;
    fn add(self, rhs: Type) -> Type {
        match rhs {
            Type::int(i) => Type::int(i),
            Type::nnint(i) => Type::nnint(i),
            Type::fract(i) => Type::fract(i),
            Type::bool(i) => Type::bool(i),
            Type::char(i) => Type::char(i),
            _ => Type::none,
        }
    }
}

impl Sub for Type {
    type Output = Type;
    fn sub(self, rhs: Type) -> Type {
        match rhs {
            Type::int(i) => Type::int(i),
            Type::nnint(i) => Type::nnint(i),
            Type::fract(i) => Type::fract(i),
            Type::bool(i) => Type::bool(i),
            Type::char(i) => Type::char(i),
            _ => Type::none,
        }
    }
}

impl Mul for Type {
    type Output = Type;
    fn mul(self, rhs: Type) -> Type {
        match rhs {
            Type::int(i) => Type::int(i),
            Type::nnint(i) => Type::nnint(i),
            Type::fract(i) => Type::fract(i),
            Type::bool(i) => Type::bool(i),
            Type::char(i) => Type::char(i),
            _ => Type::none,
        }
    }
}

impl Div for Type {
    type Output = Type;
    fn div(self, rhs: Type) -> Type {
        match rhs {
            Type::int(i) => Type::int(i),
            Type::nnint(i) => Type::nnint(i),
            Type::fract(i) => Type::fract(i),
            Type::bool(i) => Type::bool(i),
            Type::char(i) => Type::char(i),
            _ => Type::none,
        }
    }
}

impl Shl for Type {
    type Output = Type;
    fn shl(self, rhs: Type) -> Type {
        match rhs {
            Type::int(i) => Type::int(i),
            Type::nnint(i) => Type::nnint(i),
            Type::fract(i) => Type::fract(i),
            Type::bool(i) => Type::bool(i),
            Type::char(i) => Type::char(i),
            _ => Type::none,
        }
    }
}

impl Shr for Type {
    type Output = Type;
    fn shr(self, rhs: Type) -> Type {
        match rhs {
            Type::int(i) => Type::int(i),
            Type::nnint(i) => Type::nnint(i),
            Type::fract(i) => Type::fract(i),
            Type::bool(i) => Type::bool(i),
            Type::char(i) => Type::char(i),
            _ => Type::none,
        }
    }
}

impl BitAnd for Type {
    type Output = Type;
    fn bitand(self, rhs: Type) -> Type {
        match rhs {
            Type::int(i) => Type::int(i),
            Type::nnint(i) => Type::nnint(i),
            Type::fract(i) => Type::fract(i),
            Type::bool(i) => Type::bool(i),
            Type::char(i) => Type::char(i),
            _ => Type::none,
        }
    }
}

impl BitXor for Type {
    type Output = Type;
    fn bitxor(self, rhs: Type) -> Type {
        match rhs {
            Type::int(i) => Type::int(i),
            Type::nnint(i) => Type::nnint(i),
            Type::fract(i) => Type::fract(i),
            Type::bool(i) => Type::bool(i),
            Type::char(i) => Type::char(i),
            _ => Type::none,
        }
    }
}
impl BitOr for Type {
    type Output = Type;
    fn bitor(self, rhs: Type) -> Type {
        match rhs {
            Type::int(i) => Type::int(i),
            Type::nnint(i) => Type::nnint(i),
            Type::fract(i) => Type::fract(i),
            Type::bool(i) => Type::bool(i),
            Type::char(i) => Type::char(i),
            _ => Type::none,
        }
    }
}
