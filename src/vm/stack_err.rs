use crate::{
    stack::StackError,
    vm::{InterpretError, InterpretErrorType},
};

impl From<StackError> for InterpretError {
    fn from(value: StackError) -> Self {
        use InterpretErrorType::*;
        use StackError::*;
        match value {
            StackOverflow => Self {
                msg: "Stack overflow".to_string(),
                error: Runtime,
            },
            StackUnderflow => Self {
                msg: "Stack underflow".to_string(),
                error: Runtime,
            },
        }
    }
}
