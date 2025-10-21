use core::panic;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Type {
    #[default]
    Unknown,
    Int,                                  // i32
    Double,                               // double
    Unit,                                 // i8 with value 0 in LLVM
    Bool,                                 // i1
    FixedArray(Box<Type>, Option<usize>), // [T]
}

impl Type {
    pub fn llvm(&self) -> String {
        match self {
            Type::Unknown => "i8".into(),
            Type::Int => "i32".into(),
            Type::Double => "double".into(),
            Type::Unit => "i8".into(),
            Type::Bool => "i1".into(),
            Type::FixedArray(elem, Some(n)) => format!("[{} x {}]", n, elem.llvm()),
            Type::FixedArray(_elem, None) => {
                panic!("Cannot get LLVM type of FixedArray with unknown length")
            }
        }
    }

    pub fn align(&self) -> usize {
        match self {
            Type::Unknown => 1,
            Type::Int => 4,
            Type::Double => 8,
            Type::Unit => 1,
            Type::Bool => 1,
            Type::FixedArray(elem, _n) => elem.align(),
        }
    }

    pub fn size_bytes(&self) -> Option<usize> {
        match self {
            Type::Unknown => None,
            Type::Int => Some(4),
            Type::Double => Some(8),
            Type::Unit => Some(1),
            Type::Bool => Some(1),
            Type::FixedArray(elem, Some(n)) => elem.size_bytes().map(|s| s * *n),
            Type::FixedArray(_elem, None) => {
                panic!("Cannot get size of FixedArray with unknown length")
            }
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Unknown => write!(f, "unknown"),
            Type::Int => write!(f, "Int"),
            Type::Double => write!(f, "Double"),
            Type::Unit => write!(f, "Unit"),
            Type::Bool => write!(f, "Bool"),
            Type::FixedArray(t, Some(n)) => write!(f, "[{}; {}]", t, n),
            Type::FixedArray(_t, None) => {
                panic!("FixedArray with unknown length cannot be displayed")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Value {
    pub repr: String, // LLVM textual operand: "%t3", "1", "%a", etc.
    pub ty: Type,
    pub is_addr: bool, // true if 'repr' is an address (T*), false if it is an SSA value of type T
}

impl Value {
    pub fn new_val(repr: impl Into<String>, ty: Type) -> Self {
        Self {
            repr: repr.into(),
            ty,
            is_addr: false,
        }
    }
    pub fn new_addr(repr: impl Into<String>, ty: Type) -> Self {
        Self {
            repr: repr.into(),
            ty,
            is_addr: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValueObj {
    pub name: String,
    pub val: Value,
    pub mutable: bool,
}

pub fn types_compatible(target: &Type, source: &Type) -> bool {
    match (target, source) {
        (Type::Unknown, _) | (_, Type::Unknown) => true,
        (Type::Int, Type::Int) => true,
        (Type::Double, Type::Double) => true,
        (Type::Unit, Type::Unit) => true,
        (Type::Bool, Type::Bool) => true,
        (Type::FixedArray(t1, None), Type::FixedArray(t2, Some(_))) => types_compatible(t1, t2),
        (Type::FixedArray(t1, n1), Type::FixedArray(t2, n2)) => {
            n1 == n2 && types_compatible(t1, t2)
        }
        _ => false,
    }
}
