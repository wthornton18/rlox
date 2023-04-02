use crate::constants::Constants;
use crate::program::OpCode;

pub fn disassemble_instruction(
    op: &(OpCode, usize),
    constants: &Constants,
    ip: usize,
    prefix: &str,
) {
    use OpCode::*;
    let s = match op.0 {
        OpReturn => format!("OP_RETURN"),
        OpConstant(idx) => format!(
            "{prefix}{ip} OP_CONSTANT: {idx} => {constant:?}",
            constant = constants[idx.clone()]
        ),
        OpNegate => format!("OP_NEGATE"),
        OpAdd => format!("OP_ADD"),
        OpMultiply => format!("OP_MULTIPLY"),
        OpDivide => format!("OP_DIVIDE"),
        OpSubtract => format!("OP_SUBTRACT"),
        OpNil => format!("OP_NIL"),
        OpFalse => format!("OP_FALSE"),
        OpTrue => format!("OP_TRUE"),
        OpNot => format!("OP_NOT"),
    };
    println!("{prefix}{ip} {s} {line}", line = op.1);
}
