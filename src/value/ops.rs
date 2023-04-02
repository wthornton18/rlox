use super::core::*;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub enum OpErr {
    BooleanErr,
    FloatBooleanErr,
    BooleanFloatErr,
}

pub struct AdditionErr(pub OpErr);
pub struct SubtractionErr(pub OpErr);

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

impl Add<Value> for Value {
    type Output = Result<Self, AdditionErr>;
    fn add(self, rhs: Value) -> Self::Output {
        use OpErr::*;
        use Value::*;
        match (self, rhs) {
            (Float(s), Float(v)) => Ok(Float(s + v)),
            (Float(_), Boolean(_)) => Err(FloatBooleanErr)?,
            (Boolean(_), Float(_)) => Err(BooleanFloatErr)?,
            (Boolean(_), Boolean(_)) => Err(BooleanErr)?,
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
            (Float(_), Boolean(_)) => Err(FloatBooleanErr)?,
            (Boolean(_), Float(_)) => Err(BooleanFloatErr)?,
            (Boolean(_), Boolean(_)) => Err(BooleanErr)?,
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
            (Float(_), Boolean(_)) => Err(FloatBooleanErr)?,
            (Boolean(_), Float(_)) => Err(BooleanFloatErr)?,
            (Boolean(_), Boolean(_)) => Err(BooleanErr)?,
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
            (Float(_), Boolean(_)) => Err(FloatBooleanErr)?,
            (Boolean(_), Float(_)) => Err(BooleanFloatErr)?,
            (Boolean(_), Boolean(_)) => Err(BooleanErr)?,
        }
    }
}

impl Neg for Value {
    type Output = Value;
    fn neg(self) -> Self::Output {
        use Value::*;

        match self {
            Float(f) => Float(-f),
            Boolean(b) => Boolean(!b),
        }
    }
}
