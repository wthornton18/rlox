use super::core::*;
use std::ops::{Add, Div, Mul, Neg, Not, Sub};

pub enum OpErr {
    Internal,
}

pub struct AdditionErr(pub OpErr);
pub struct SubtractionErr(pub OpErr);
pub struct NegErr(pub OpErr);

pub struct DivisionErr {
    pub op_err: Option<OpErr>,
    pub divide_by_zero: bool,
}
pub struct MultiplyErr(pub OpErr);

impl From<OpErr> for AdditionErr {
    fn from(value: OpErr) -> Self {
        Self(value)
    }
}

impl From<OpErr> for SubtractionErr {
    fn from(value: OpErr) -> Self {
        Self(value)
    }
}

impl From<OpErr> for DivisionErr {
    fn from(value: OpErr) -> Self {
        Self {
            op_err: Some(value),
            divide_by_zero: false,
        }
    }
}
impl DivisionErr {
    fn divide_by_zero_err() -> Self {
        Self {
            op_err: None,
            divide_by_zero: true,
        }
    }
}

impl From<OpErr> for MultiplyErr {
    fn from(value: OpErr) -> Self {
        Self(value)
    }
}

impl From<OpErr> for NegErr {
    fn from(value: OpErr) -> Self {
        Self(value)
    }
}

impl Add<Value> for Value {
    type Output = Result<Self, AdditionErr>;
    fn add(self, rhs: Value) -> Self::Output {
        use OpErr::*;
        use Value::*;
        match (self, rhs) {
            (Float(s), Float(v)) => Ok(Float(s + v)),
            _ => Err(Internal)?,
        }
    }
}
impl Sub<Value> for Value {
    type Output = Result<Self, SubtractionErr>;
    fn sub(self, rhs: Value) -> Self::Output {
        use OpErr::*;
        use Value::*;
        match (self, rhs) {
            (Float(s), Float(v)) => Ok(Float(s - v)),
            _ => Err(Internal)?,
        }
    }
}

impl Div<Value> for Value {
    type Output = Result<Self, DivisionErr>;
    fn div(self, rhs: Value) -> Self::Output {
        use OpErr::*;
        use Value::*;

        match (self, rhs) {
            (Float(s), Float(v)) => {
                if v == 0.0 {
                    Err(DivisionErr::divide_by_zero_err())
                } else {
                    Ok(Float(s / v))
                }
            }
            _ => Err(Internal)?,
        }
    }
}

impl Mul<Value> for Value {
    type Output = Result<Self, MultiplyErr>;
    fn mul(self, rhs: Value) -> Self::Output {
        use OpErr::*;
        use Value::*;
        match (self, rhs) {
            (Float(s), Float(v)) => Ok(Float(s * v)),
            _ => Err(Internal)?,
        }
    }
}

impl Neg for Value {
    type Output = Result<Self, NegErr>;
    fn neg(self) -> Self::Output {
        use OpErr::Internal;
        use Value::Float;

        match self {
            Float(f) => Ok(Float(-f)),
            _ => Err(Internal)?,
        }
    }
}

impl Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        use Value::*;
        match self {
            Boolean(b) => Boolean(!b),
            Nil => Boolean(true),
            Float(f) => {
                if f == 0.0 {
                    Boolean(true)
                } else {
                    Boolean(false)
                }
            }
        }
    }
}
