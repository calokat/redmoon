use ordered_float::OrderedFloat;
use crate::function::Function;
use crate::table::UserTable;
use crate::native_function::NativeFunction;
use std::fmt::Display;
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Value {
    ValList(Vec<Value>),
    Boolean(bool),
    Number(OrderedFloat<f32>),
    String(String),
    Nil,
    FunctionDef(Function),
    NativeFunctionDef(NativeFunction),
    Table(UserTable),
    // Used when interpreting break statements. Can only be created by the runtime, not the user
    Interrupt,

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
            },
            Value::Interrupt => panic!("Unprintable value")
        }
    }
}