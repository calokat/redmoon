use std::{
    num::ParseFloatError,
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
                Number::Int(i) => Number::Float(OrderedFloat(i as f64 $op *f)),
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
        number_op!(self, rhs, /)
    }
}

impl FromStr for Number {
    type Err = ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result: Result<OrderedFloat<f64>, ParseFloatError> = s.parse();
        match result {
            Ok(number) => {
                if number == number.floor() {
                    return Ok(Number::Int(*number as i64));
                } else {
                    return Ok(Number::Float(number));
                }
            }
            Err(e) => return Err(e),
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Float(n) => write!(f, "{n}"),
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
