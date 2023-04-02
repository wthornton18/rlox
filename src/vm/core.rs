use crate::constants::*;
#[cfg(feature = "tracing")]
use crate::disassemble::*;
use crate::program::*;
use crate::stack::*;
use crate::value::*;

#[derive(Default)]
pub struct VM {
    program: Program,
    constants: Constants,
    ip: usize,
    stack: Stack,
}

#[derive(Debug, Clone, Copy)]
pub enum InterpretErrorType {
    Runtime,
    Compiler,
}
#[derive(Debug, Clone)]

pub struct InterpretError {
    pub msg: String,
    pub error: InterpretErrorType,
}

pub type InterpretResult = Result<(), InterpretError>;

impl VM {
    pub fn new(program: Program, constants: Constants) -> Self {
        Self {
            program,
            constants,
            ..Default::default()
        }
    }

    pub fn step(&mut self) -> InterpretResult {
        use OpCode::*;

        let op = self.program[self.ip];

        let res = match op.0 {
            OpConstant(idx) => {
                self.stack.push(self.constants[idx])?;
            }
            OpReturn => {
                let constant = self.stack.pop()?;
            }
            OpNegate => {
                let val = self.stack.pop()?;
                self.stack.push((-val)?)?;
            }
            OpAdd => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push((a + b)?)?;
            }
            OpSubtract => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push((a - b)?)?;
            }
            OpDivide => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push((a / b)?)?;
            }
            OpMultiply => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push((a * b)?)?;
            }
            OpNil => self.stack.push(Value::Nil)?,
            OpFalse => self.stack.push(Value::Boolean(false))?,
            OpTrue => self.stack.push(Value::Boolean(true))?,
            OpNot => {
                let a = self.stack.pop()?;
                self.stack.push(!a)?;
            }
        };

        #[cfg(feature = "tracing")]
        {
            println!("\n");
            println!("==VM==");
            disassemble_instruction(&op, &self.constants, self.ip, "\t");
            println!("\t{}", self.stack);
            println!("\n");
        }

        self.ip += 1;
        Ok(())
    }
}

impl Iterator for VM {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        if self.ip == self.program.len() {
            return None;
        }
        match self.step() {
            Ok(()) => Some(()),
            Err(err) => {
                #[cfg(feature = "tracing")]
                println!("{:?}", err);
                None
            }
        }
    }
}
