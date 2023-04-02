use crate::constants::Constants;
use crate::program::OpCode;

pub fn disassemble_instruction(
    op: &(OpCode, usize),
    constants: &Constants,
    ip: usize,
    prefix: &str,
) {
    println!("{prefix}{ip} {op:?} {line}", op = op.0, line = op.1);
}
