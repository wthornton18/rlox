#![allow(dead_code)]
#![allow(unused_variables)]

mod compiler;
mod constants;
mod disassemble;
mod program;
mod stack;
mod token;
mod tokenizer;
mod value;
mod vm;
use std::time::Instant;

use compiler::Compiler;
use tokenizer::Tokenizer;
use vm::VM;

fn main() -> Result<(), ()> {
    let source_code = include_str!("../numbers.rlox");
    let chars = source_code.chars().into_iter().collect::<Vec<_>>();
    let tokenizer = Tokenizer::new(&chars);
    let mut compiler = Compiler::new(&chars, tokenizer);
    compiler.compile();

    if !compiler.had_error {
        let vm = VM::new(compiler.program, compiler.constants);
        let t = Instant::now();
        for ir in vm {}
        println!("{t:?}", t = (Instant::now() - t));
        return Ok(());
    } else {
        println!("Error compiling program");
        for err in compiler.errors {
            println!("")
        }
        return Err(());
    }
}
