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

#[derive(Clone, Copy, Hash)]
pub enum Number {
    Int(i64),
    Float(OrderedFloat<f64>),
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        let n1 = self.try_into_int().unwrap_or(*self);
        let n2 = other.try_into_int().unwrap_or(*other);

        match n1 {
            Number::Int(i1) => match n2 {
                Number::Int(i2) => i1 == i2,
                // If n2 could become an int, it would have in try_to_int()
                Number::Float(_) => false,
            },
            Number::Float(f1) => match n2 {
                Number::Int(_) => false,
                Number::Float(f2) => f1 == f2,
            },
        }
    }
}

impl Eq for Number {}

impl PartialOrd for Number {
    fn ge(&self, other: &Self) -> bool {
        let (f1, f2) = (self.into_float(), other.into_float());
        return f1 >= f2;
    }

    fn gt(&self, other: &Self) -> bool {
        let (f1, f2) = (self.into_float(), other.into_float());
        return f1 > f2;
    }

    fn le(&self, other: &Self) -> bool {
        let (f1, f2) = (self.into_float(), other.into_float());
        return f1 <= f2;
    }

    fn lt(&self, other: &Self) -> bool {
        let (f1, f2) = (self.into_float(), other.into_float());
        return f1 < f2;
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let (f1, f2) = (self.into_float(), other.into_float());
        return f1.partial_cmp(&f2);
    }
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
                Ok(number) => {
                    if number == number.floor() {
                        return Ok(Number::Int(number as i64));
                    }
                    Ok(Number::Float(number.into()))
                }
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

    fn try_into_int(self) -> Option<Self> {
        match self {
            Number::Int(_) => Some(self),
            Number::Float(n) => {
                if n.floor() == *n {
                    return Some(Number::Int(*n as i64));
                } else {
                    return None;
                }
            }
        }
    }

    fn into_float(self) -> f64 {
        match self {
            Number::Int(i) => i as f64,
            Number::Float(f) => *f,
        }
    }
}
