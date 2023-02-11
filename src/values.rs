// use std::collections::HashMap;
use ordered_float::OrderedFloat;
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Value {
    Boolean(bool),
    Number(OrderedFloat<f32>),
    String(String),
    Nil,
}