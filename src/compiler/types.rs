use std::fmt::Display;

use crate::tokenizer::OTokenResult;

#[derive(Debug, Clone)]
pub struct CompilerErr {
    token: OTokenResult,
    content: String,
}

impl CompilerErr {
    pub fn new(token: &OTokenResult, content: &str) -> Self {
        Self {
            token: token.to_owned(),
            content: content.to_owned(),
        }
    }
}

impl Display for CompilerErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (suffix, line) = match self.token.clone() {
            None => (String::from(" at end"), 0),
            Some(Ok(t)) => (
                format!(" at ({start}, {length})", start = t.pos, length = t.length),
                t.line,
            ),
            Some(Err(t)) => (format!(""), t.line),
        };

        write!(
            f,
            "[line {line}]: Error {suffix}: {msg}",
            msg = self.content
        )
    }
}
pub type CompilerErrors = Vec<CompilerErr>;
