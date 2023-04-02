#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    OpReturn,

    //BINARY OPERATIONS
    OpAdd,
    OpNegate,
    OpSubtract,
    OpMultiply,
    OpDivide,

    OpConstant(usize),
    OpNil,
    OpTrue,
    OpFalse,
    OpNot,
}

pub type Instruction = (OpCode, usize);
pub type Program = Vec<Instruction>;
