const STACK_SIZE: usize = 255;
use crate::value::*;

pub struct Stack {
    arr: [Value; STACK_SIZE],
    sp: usize,
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            arr: [Value::Float(0.0); STACK_SIZE],
            sp: 0,
        }
    }
}

pub enum StackError {
    StackUnderflow,
    StackOverflow,
}

impl std::fmt::Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in 0..self.sp {
            write!(f, "{val:?}, ", val = self.arr[i])?
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl Stack {
    pub fn push(&mut self, value: Value) -> Result<(), StackError> {
        self.arr[self.sp] = value;
        self.sp += 1;
        if self.sp == STACK_SIZE {
            Err(StackError::StackOverflow)
        } else {
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Result<Value, StackError> {
        if self.sp == 0 {
            Err(StackError::StackUnderflow)
        } else {
            self.sp -= 1;
            Ok(self.arr[self.sp])
        }
    }
}
