use crate::{
    constants::Constants,
    program::{OpCode, Program},
    tokenizer::{OTokenResult, Token, TokenType, Tokenizer},
    value::Value,
};

use super::types::CompilerErrors;
use super::{
    precedence::{get_rule, Precedence},
    types::CompilerErr,
};

pub struct Compiler<'a> {
    source: &'a [char],
    tokenizer: Tokenizer<'a>,
    panic_mode: bool,
    pub had_error: bool,
    current_token: Option<Token>,
    previous_token: Option<Token>,
    pub constants: Constants,
    pub program: Program,
    pub errors: CompilerErrors,
}

impl<'source> Compiler<'source> {
    pub fn new(source: &'source [char], tokenizer: Tokenizer<'source>) -> Self {
        Self {
            source,
            tokenizer,
            panic_mode: false,
            had_error: false,
            current_token: None,
            previous_token: None,
            program: vec![],
            errors: vec![],
            constants: Constants::new(),
        }
    }

    pub fn compile(&mut self) {
        self.advance();
        self.expression();
        self.advance_end();
        self.program.push((OpCode::OpReturn, 0));
    }

    pub fn grouping(&mut self) {
        self.expression();
        self.advance_match(TokenType::RightParen, "Expect ')' after expression");
    }

    pub fn unary(&mut self) {
        use OpCode::{OpNegate, OpNot};
        use Precedence::Unary;
        use TokenType::{Bang, Minus};

        let previous_token = self.previous_token.clone();

        self.parse_precedence(Unary);

        match previous_token {
            Some(Token {
                token_type: Minus,
                line,
                ..
            }) => self.program.push((OpNegate, line)),
            Some(Token {
                token_type: Bang,
                line,
                ..
            }) => self.program.push((OpNot, line)),
            _ => {}
        }
    }

    pub fn binary(&mut self) {
        use OpCode::{OpAdd, OpDivide, OpMultiply, OpSubtract};
        use TokenType::{Minus, Plus, Slash, Star};

        if let Some(token) = self.previous_token.clone() {
            let rule = get_rule(token.token_type);

            self.parse_precedence(rule.precedence + 1);

            match token.token_type {
                Plus => self.program.push((OpAdd, token.line)),
                Minus => self.program.push((OpSubtract, token.line)),
                Star => self.program.push((OpMultiply, token.line)),
                Slash => self.program.push((OpDivide, token.line)),
                _ => {}
            }
        }
    }

    pub fn number(&mut self) {
        use OpCode::OpConstant;

        match self.previous_token {
            Some(Token {
                pos, length, line, ..
            }) => {
                let num = self.source[pos..(pos + length)]
                    .iter()
                    .collect::<String>()
                    .parse::<f64>();
                let num = match num {
                    Ok(num) => num,
                    Err(_) => {
                        self.error("Failed to parse number, defaulting to 0");
                        0 as f64
                    }
                };

                let idx = self.constants.push(Value::Float(num));
                self.program.push((OpConstant(idx), line))
            }
            None => {}
        }
    }

    pub fn literal(&mut self) {
        use OpCode::{OpFalse, OpNil, OpTrue};
        use TokenType::{FalseIdent, Nil, TrueIdent};

        if let Some(token) = self.previous_token.clone() {
            match token.token_type {
                FalseIdent => self.program.push((OpFalse, token.line)),
                TrueIdent => self.program.push((OpTrue, token.line)),
                Nil => self.program.push((OpNil, token.line)),
                _ => {}
            }
        }
    }
}

impl<'source> Compiler<'source> {
    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment)
    }

    fn error_at_current(&mut self, msg: &str) {
        let current_token: OTokenResult = self.current_token.clone().map(|t| Ok(t));
        self.error_at(&current_token, msg)
    }

    fn error_at(&mut self, token: &OTokenResult, msg: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        self.errors.push(CompilerErr::new(token, msg));
        self.had_error = true;
    }

    fn error(&mut self, msg: &str) {
        let previous_token: OTokenResult = self.previous_token.clone().map(|t| Ok(t));
        self.error_at(&previous_token, msg)
    }

    fn advance_match(&mut self, expected_token_type: TokenType, error_msg: &str) {
        match self.current_token {
            Some(t) => {
                if t.token_type == expected_token_type {
                    self.advance();
                    return;
                };
            }
            None => {}
        }

        self.error_at_current(error_msg);
    }

    fn advance_end(&mut self) {
        self.advance();
        match self.current_token {
            None => {}
            Some(_) => self.error_at_current("Expected end of token stream"),
        }
    }

    fn advance(&mut self) {
        self.previous_token = self.current_token;
        loop {
            let current_token = self.tokenizer.next();

            match current_token {
                Some(Ok(t)) => {
                    self.current_token = Some(t);
                    break;
                }
                Some(Err(t)) => self.error_at(&Some(Err(t.clone())), &t.content()),
                None => {
                    self.current_token = None;
                    break;
                }
            }
        }
        #[cfg(feature = "tracing")]
        {
            println!("\t==compiler==");
            println!("\t\tcurrent token: {ct:?}", ct = self.current_token);
            println!("\t\tprevious token: {pt:?}", pt = self.previous_token);
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        if let Some(token) = self.previous_token {
            match get_rule(token.token_type).prefix {
                Some(op) => op(self),
                None => self.error("Expect expression."),
            }
        }

        while self.current_token.is_some() {
            let current_token = self.current_token.unwrap();
            let current_token_precedence = get_rule(current_token.token_type).precedence;
            if precedence > current_token_precedence {
                return;
            }

            self.advance();
            if let Some(previous_token) = self.previous_token {
                if let Some(infix) = get_rule(previous_token.token_type).infix {
                    infix(self);
                }
            }
        }
    }
}
