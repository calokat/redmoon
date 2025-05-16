use std::{
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

use std::fmt::Display;

use ordered_float::{OrderedFloat, Pow};

macro_rules! number_op {
    ($a:ident, $b:ident, $op: tt) => {
                match $a {
            Number::Int(i) => match $b {
                Number::Int(ri) => Number::Int(i $op ri),
                Number::Float(f) => Number::Float(OrderedFloat(i as f64 $op *f)),
            },
            Number::Float(f) => match $b {
                Number::Int(i) => Number::Float(OrderedFloat(*f $op i as f64)),
                Number::Float(rf) => Number::Float(f $op rf),
            },
        }

    };
}

#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum Number {
    Int(i64),
    Float(OrderedFloat<f64>),
}

impl Add for Number {
    type Output = Number;
    fn add(self, rhs: Self) -> Self::Output {
        number_op!(self, rhs, +)
    }
}

impl Sub for Number {
    type Output = Number;
    fn sub(self, rhs: Self) -> Self::Output {
        number_op!(self, rhs, -)
    }
}

impl Mul for Number {
    type Output = Number;
    fn mul(self, rhs: Self) -> Self::Output {
        number_op!(self, rhs, *)
    }
}

impl Div for Number {
    type Output = Number;
    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Number::Int(i) => match rhs {
                Number::Int(ri) => {
                    if i % ri == 0 {
                        Number::Int(i / ri)
                    } else {
                        Number::Float(OrderedFloat(i as f64 / ri as f64))
                    }
                }
                Number::Float(f) => Number::Float(OrderedFloat(i as f64 / *f)),
            },
            Number::Float(f) => match rhs {
                Number::Int(i) => Number::Float(OrderedFloat(*f / i as f64)),
                Number::Float(rf) => Number::Float(f / rf),
            },
        }
    }
}

impl FromStr for Number {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = s.parse::<i64>();
        match result {
            Ok(number) => Ok(Number::Int(number)),
            Err(_) => match s.parse::<f64>() {
                Ok(number) => Ok(Number::Float(number.into())),
                Err(e) => Err(e.to_string()),
            },
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Float(n) => {
                let suffix = if n.floor() == **n { ".0" } else { "" };
                write!(f, "{n}{suffix}")
            }
            Number::Int(i) => write!(f, "{i}"),
        }
    }
}

impl Number {
    pub fn floor(self) -> Self {
        match self {
            Number::Float(f) => Number::Float(OrderedFloat(f.floor())),
            Number::Int(_) => return self,
        }
    }

    pub fn pow(self, rhs: Self) -> Self {
        match self {
            Number::Int(i) => match rhs {
                Number::Float(f) => Number::Float(OrderedFloat((i as f64).pow(*f))),
                Number::Int(ri) => Number::Float(OrderedFloat((i as f64).pow(ri as f64))),
            },
            Number::Float(f) => match rhs {
                Number::Float(rf) => Number::Float((f).pow(rf)),
                Number::Int(i) => Number::Float(OrderedFloat((*f).pow(i as f64))),
            },
        }
    }
}
