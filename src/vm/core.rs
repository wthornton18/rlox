use crate::constants::*;
#[cfg(feature = "tracing")]
use crate::disassemble::*;
use crate::program::*;
use crate::stack::*;
use crate::value::*;
use std::cmp::Ordering;

#[derive(Default)]
pub struct VM {
    program: Program,
    constants: Constants,
    strings: Vec<String>,
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
impl InterpretError {
    fn runtime_error(msg: &str) -> Self {
        Self {
            msg: msg.to_owned(),
            error: InterpretErrorType::Runtime,
        }
    }
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
        use Object::*;
        use OpCode::*;
        use Value::*;

        let op = self.program[self.ip];

        let res = match op.0 {
            OpConstant(idx) => {
                self.stack.push(self.constants[idx].clone())?;
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
                if let (Some(a), Some(b)) = (a.get_string_ref(), b.get_string_ref()) {
                    let mut new_string = self.strings[a].clone();
                    new_string.extend(self.strings[b].chars());
                    self.strings.push(new_string);
                    self.constants
                        .push(Value::new_string(self.strings.len() - 1));
                } else {
                    self.stack.push((a + b)?)?;
                };
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
                if let Some(a) = a.get_string_ref() {
                    let b = !(self.strings[a].len() == 0);
                    self.stack.push(Boolean(b))?;
                } else {
                    self.stack.push(!a)?;
                }
            }
            OpGreater | OpGreaterEqual | OpLess | OpLessEqual => {
                use Ordering::*;
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                let res = match (a.partial_cmp(&b), op.0) {
                    (None, _) => Err(InterpretError::runtime_error(
                        "Cannot compare the two types",
                    ))?,
                    (Some(Equal), OpGreaterEqual | OpLessEqual)
                    | (Some(Less), OpLess | OpLessEqual)
                    | (Some(Greater), OpGreater | OpGreaterEqual) => true,
                    _ => false,
                };
            }
            OpEqual => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;

                if let (Some(a), Some(b)) = (a.get_string_ref(), b.get_string_ref()) {
                    let res = self.strings[a] == self.strings[b];
                    self.stack.push(Value::Boolean(res))?;
                } else {
                    self.stack.push(Value::Boolean(a == b))?
                }
            }
            OpNotEqual => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push(Value::Boolean(a != b))?
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
