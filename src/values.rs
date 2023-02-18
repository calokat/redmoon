use ordered_float::OrderedFloat;
use crate::function::Function;

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Value {
    Boolean(bool),
    Number(OrderedFloat<f32>),
    String(String),
    Nil,
    FunctionDef(Function),
}