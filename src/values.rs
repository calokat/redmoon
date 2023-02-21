use ordered_float::OrderedFloat;
use crate::function::Function;
use crate::table::UserTable;
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Value {
    ValList(Vec<Value>),
    Boolean(bool),
    Number(OrderedFloat<f32>),
    String(String),
    Nil,
    FunctionDef(Function),
    Table(UserTable)
}