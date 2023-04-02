use crate::value::ops::{AdditionErr, DivisionErr, MultiplyErr, NegErr, SubtractionErr};

use super::{InterpretError, InterpretErrorType};

impl From<AdditionErr> for InterpretError {
    fn from(value: AdditionErr) -> Self {
        use InterpretErrorType::*;
        match value.0 {
            _ => Self {
                msg: "Addition err".to_string(),
                error: Runtime,
            },
        }
    }
}

impl From<SubtractionErr> for InterpretError {
    fn from(value: SubtractionErr) -> Self {
        use InterpretErrorType::*;
        match value.0 {
            _ => Self {
                msg: "Subtraction err".to_string(),
                error: Runtime,
            },
        }
    }
}

impl From<DivisionErr> for InterpretError {
    fn from(value: DivisionErr) -> Self {
        use InterpretErrorType::*;
        Self {
            msg: "Divison err".to_string(),
            error: Runtime,
        }
    }
}

impl From<MultiplyErr> for InterpretError {
    fn from(value: MultiplyErr) -> Self {
        use InterpretErrorType::*;
        match value.0 {
            _ => Self {
                msg: "Multiply err".to_string(),
                error: Runtime,
            },
        }
    }
}

impl From<NegErr> for InterpretError {
    fn from(value: NegErr) -> Self {
        use InterpretErrorType::*;
        match value.0 {
            _ => Self {
                msg: "Negation err".to_string(),
                error: Runtime,
            },
        }
    }
}
