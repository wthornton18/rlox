#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    LeftParen,
    RightParen,

    LeftBrace,
    RightBrace,

    Comma,
    Dot,
    Minus,
    Plus,

    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    StringLiteral,
    NumericLiteral,

    And,
    Class,
    Else,
    FalseIdent,
    For,
    Func,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    TrueIdent,
    Var,
    While,

    Error,
}
#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub token_type: TokenType,
    pub pos: usize,
    pub length: usize,
    pub line: usize,
}
impl Token {
    fn new(token_type: TokenType, pos: usize, length: usize, line: usize) -> Self {
        Self {
            token_type,
            pos,
            length,
            line,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenErr {
    content: String,
    pub line: usize,
}

impl TokenErr {
    pub fn content(&self) -> String {
        self.content.clone()
    }
    pub fn line(&self) -> usize {
        self.line
    }
}

impl TokenErr {
    fn new(msg: String, line: usize) -> Self {
        Self { content: msg, line }
    }
}

pub type TokenResult = Result<Token, TokenErr>;
pub type OTokenResult = Option<TokenResult>;

pub struct Tokenizer<'a> {
    source: &'a [char],
    line: usize,
    removed_chars: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a [char]) -> Self {
        Self {
            source,
            line: 0,
            removed_chars: 0,
        }
    }

    fn eof(&self) -> bool {
        self.source.is_empty()
    }

    fn eof_n(&self, n: usize) -> bool {
        self.source.len() <= n
    }

    fn advance_while<P>(&mut self, mut predicate: P) -> usize
    where
        P: FnMut(&char) -> bool,
    {
        let mut n = 0;
        while n < self.source.len() && predicate(&self.source[n]) {
            n += 1;
        }
        self.advance_n(n);
        n
    }

    fn advance_n(&mut self, n: usize) {
        self.removed_chars += n;
        self.source = &self.source[n..];
    }

    fn advance(&mut self) {
        self.removed_chars += 1;
        self.source = &self.source[1..];
    }

    fn peak(&self) -> char {
        self.source[0]
    }

    fn peak_n(&self, n: usize) -> char {
        self.source[n]
    }

    fn peak_match(&self, c: char) -> bool {
        if self.eof() || self.source.len() - 1 == 0 {
            false
        } else {
            self.source[1] == c
        }
    }

    fn trim_left(&mut self) {
        while !self.eof() {
            match self.peak() {
                ' ' | '\r' | '\t' => self.advance(),
                '\n' => {
                    self.advance();
                    self.line += 1
                }
                '/' => {
                    if self.peak_match('/') {
                        self.advance_while(|x| *x != '\n');
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    fn single_char_token(&mut self) -> Option<Token> {
        use TokenType::*;
        let token = match self.peak() {
            '(' => Some(self.make_token(LeftParen, 1)),
            ')' => Some(self.make_token(RightParen, 1)),
            '{' => Some(self.make_token(LeftBrace, 1)),
            '}' => Some(self.make_token(RightBrace, 1)),
            ',' => Some(self.make_token(Comma, 1)),
            '.' => Some(self.make_token(Dot, 1)),
            '-' => Some(self.make_token(Minus, 1)),
            '+' => Some(self.make_token(Plus, 1)),
            ';' => Some(self.make_token(Semicolon, 1)),
            '/' => Some(self.make_token(Slash, 1)),
            '*' => Some(self.make_token(Star, 1)),
            _ => None,
        };

        if let Some(_) = token {
            self.advance();
        }
        token
    }

    fn two_char_token(&mut self) -> Option<Token> {
        use TokenType::*;
        let token = match self.peak() {
            '!' => {
                if self.peak_match('=') {
                    Some(self.make_token(BangEqual, 2))
                } else {
                    Some(self.make_token(Bang, 1))
                }
            }
            '=' => {
                if self.peak_match('=') {
                    Some(self.make_token(EqualEqual, 2))
                } else {
                    Some(self.make_token(Equal, 1))
                }
            }
            '<' => {
                if self.peak_match('=') {
                    Some(self.make_token(LessEqual, 2))
                } else {
                    Some(self.make_token(Less, 1))
                }
            }
            '>' => {
                if self.peak_match('=') {
                    Some(self.make_token(GreaterEqual, 2))
                } else {
                    Some(self.make_token(Greater, 1))
                }
            }
            _ => None,
        };
        if let Some(ref token) = token {
            self.advance_n(token.length)
        }
        token
    }

    fn string_literal(&mut self) -> TokenResult {
        use TokenType::StringLiteral;
        let mut n = 0;
        while !self.eof_n(n + 1) && self.peak_n(n + 1) != '"' {
            if self.peak_n(n + 1) == '\n' {
                self.line += 1;
            }
            n += 1;
        }
        let token = match self.eof_n(n + 1) {
            true => Err(self.make_error_token("Undetermined string literal")),
            false => {
                n += 1;
                Ok(self.make_token(StringLiteral, n + 1))
            }
        };
        self.advance_n(n + 1);
        token
    }

    fn numeric_literal(&mut self) -> Token {
        use TokenType::NumericLiteral;

        let mut n = 0;
        while !self.eof_n(n + 1) && self.peak_n(n + 1).is_digit(10) {
            n += 1;
        }

        if !self.eof_n(n + 1) && self.peak_n(n + 1) == '.' {
            n += 1;
            while !self.eof_n(n + 1) && self.peak_n(n + 1).is_digit(10) {
                n += 1;
            }
        }
        let token = self.make_token(NumericLiteral, n + 1);
        self.advance_n(n + 1);
        token
    }

    fn identifier(&mut self) -> Token {
        use TokenType::*;
        let mut n = 0;
        while n < self.source.len() && (self.source[n].is_alphanumeric() || self.source[n] == '_') {
            n += 1;
        }

        let token = match self.source[0..n].iter().collect::<String>().as_ref() {
            "and" => self.make_token(And, 3),
            "class" => self.make_token(Class, 4),
            "else" => self.make_token(Else, 4),
            "if" => self.make_token(If, 2),
            "nil" => self.make_token(Nil, 3),
            "or" => self.make_token(Or, 2),
            "print" => self.make_token(Print, 5),
            "return" => self.make_token(Return, 6),
            "super" => self.make_token(Super, 5),
            "var" => self.make_token(Var, 3),
            "while" => self.make_token(While, 5),
            "fun" => self.make_token(Func, 3),
            "false" => self.make_token(FalseIdent, 5),
            "for" => self.make_token(For, 3),
            "this" => self.make_token(This, 4),
            "true" => self.make_token(TrueIdent, 4),
            _ => self.make_token(Identifier, n),
        };
        self.advance_n(n);
        token
    }

    fn next_token(&mut self) -> OTokenResult {
        self.trim_left();
        if self.eof() {
            return None;
        }

        if let Some(token) = self.single_char_token() {
            return Some(Ok(token));
        }

        if let Some(token) = self.two_char_token() {
            return Some(Ok(token));
        }

        if self.peak() == '"' {
            let token = self.string_literal();
            return Some(token);
        }

        if self.peak().is_digit(10) {
            let token = Ok(self.numeric_literal());

            return Some(token);
        }

        if self.peak().is_alphabetic() {
            let token = Ok(self.identifier());
            return Some(token);
        }

        let unrecognised_token =
            Err(self
                .make_error_token(format!("Unrecognised token {c}", c = self.source[0]).as_ref()));
        self.advance();
        return Some(unrecognised_token);
    }

    fn make_token(&self, token_type: TokenType, len: usize) -> Token {
        Token::new(token_type, self.removed_chars, len, self.line)
    }

    fn make_error_token(&self, msg: &str) -> TokenErr {
        TokenErr::new(msg.to_owned(), self.line)
    }

    pub fn get_current_line(&self) -> usize {
        self.line
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = TokenResult;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();
        #[cfg(feature = "tracing")]
        {
            println!("\n");
            println!("\t==tokenizer==");
            println!("\t\ttoken: {token:?}");
            println!("\n");
        }
        token
    }
}
