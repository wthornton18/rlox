#[cfg(feature = "tracing")]
use crate::constants::Constants;
#[cfg(feature = "tracing")]
use crate::program::OpCode;

#[cfg(feature = "tracing")]
pub fn disassemble_instruction(
    op: &(OpCode, usize),
    constants: &Constants,
    ip: usize,
    prefix: &str,
) {
    use OpCode::*;
    match op.0 {
        OpReturn => println!("{prefix}{ip}  OP_RETURN"),
        OpConstant(idx) => println!(
            "{prefix}{ip} OP_CONSTANT: {idx} => {constant:?}",
            constant = constants[idx.clone()]
        ),
        OpNegate => println!("{prefix}{ip} OP_NEGATE"),
        OpAdd => println!("{prefix}{ip} OP_ADD"),
        OpMultiply => println!("{prefix}{ip} OP_MULTIPLY"),
        OpDivide => println!("{prefix}{ip} OP_DIVIDE"),
        OpSubtract => println!("{prefix}{ip} OP_SUBTRACT"),
    }
}
