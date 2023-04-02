use crate::tokenizer::TokenType;
use std::ops::Add;

use super::core::Compiler;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(u8)]
pub enum Precedence {
    PrecNone,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

impl Add<u8> for Precedence {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        Precedence::from((self as u8) + rhs)
    }
}

impl From<u8> for Precedence {
    fn from(value: u8) -> Self {
        use Precedence::*;
        match value {
            0 => PrecNone,
            1 => Assignment,
            2 => Or,
            3 => And,
            4 => Equality,
            5 => Comparison,
            6 => Term,
            7 => Factor,
            8 => Unary,
            9 => Call,
            _ => Primary,
        }
    }
}

type Operation<'source> = fn(&mut Compiler<'source>);

type OOperation<'source> = Option<Operation<'source>>;

pub struct Rule<'source> {
    pub prefix: OOperation<'source>,
    pub infix: OOperation<'source>,
    pub precedence: Precedence,
}

impl<'source> Rule<'source> {
    fn new(
        prefix: OOperation<'source>,
        infix: OOperation<'source>,
        precedence: Precedence,
    ) -> Self {
        Self {
            prefix,
            infix,
            precedence,
        }
    }
}

pub fn get_rule<'source>(token_type: TokenType) -> Rule<'source> {
    use Precedence::*;
    use TokenType::*;
    match token_type {
        LeftParen => Rule::new(Some(Compiler::grouping), None, PrecNone),

        Minus => Rule::new(Some(Compiler::unary), Some(Compiler::binary), Term),

        Plus => Rule::new(None, Some(Compiler::binary), Term),

        Slash | Star => Rule::new(None, Some(Compiler::binary), Factor),

        NumericLiteral => Rule::new(Some(Compiler::number), None, PrecNone),

        Nil | TrueIdent | FalseIdent => Rule::new(Some(Compiler::literal), None, PrecNone),

        Bang => Rule::new(Some(Compiler::unary), None, PrecNone),

        BangEqual | EqualEqual => Rule::new(None, Some(Compiler::binary), Equality),

        Greater | GreaterEqual | LessEqual | Less => {
            Rule::new(None, Some(Compiler::binary), Comparison)
        }

        _ => Rule::new(None, None, PrecNone),
    }
}
