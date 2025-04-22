use crate::function::Function;
use crate::gc::gc_key::GcKey;
use crate::native_function::NativeFunction;
use ordered_float::OrderedFloat;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

macro_rules! impl_op {
    ($trait:ident, $fn:ident, $op:tt) => {
        impl $trait for Value {
            type Output = Result<Value, String>;
            fn $fn(self, rhs: Self) -> Self::Output {
                match self {
                    Value::Number(a) => match rhs {
                        Value::Number(b) => Ok(Value::Number(a $op b)),
                        _ => Err("Only numbers can be added".into()),
                    },
                    _ => Err("Only numbers can be added".into()),
                }
            }
        }

    };
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Value {
    ValList(Vec<Value>),
    Boolean(bool),
    Number(OrderedFloat<f32>),
    String(String),
    Nil,
    FunctionDef(Function),
    NativeFunctionDef(NativeFunction),
    Table(GcKey),
    // Used when interpreting break statements. Can only be created by the runtime, not the user
    Interrupt,
    // Used for storing metatables in tables
    MetaKey,
    Varargs(Vec<Value>),
    VarargsIdentifier,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Boolean(b) => write!(f, "{b}"),
            Value::FunctionDef(_) => write!(f, "<function definition>"),
            Value::NativeFunctionDef(_) => write!(f, "<native function definition>"),
            Value::Nil => write!(f, "nil"),
            Value::Number(n) => write!(f, "{n}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Table(_) => write!(f, "<table>"),
            Value::ValList(vl) => {
                for v in vl.iter() {
                    if let std::fmt::Result::Err(e) = write!(f, "{v}\t") {
                        return std::fmt::Result::Err(e);
                    }
                }
                std::fmt::Result::Ok(())
            }
            Value::Interrupt => panic!("Unprintable value"),
            Value::MetaKey => panic!("Unprintable value"),
            Value::Varargs(va) => {
                for v in va.iter() {
                    if let std::fmt::Result::Err(e) = write!(f, "{v}\t") {
                        return std::fmt::Result::Err(e);
                    }
                }
                std::fmt::Result::Ok(())
            }
            Value::VarargsIdentifier => write!(f, "<varargs>"),
        }
    }
}

impl_op! {Add, add, +}

impl_op! {Sub, sub, -}

impl_op! {Mul, mul, *}

impl_op! {Div, div, /}
