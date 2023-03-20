use std::fmt::Display;

#[derive(Debug)]
pub enum LexerError {
    NoTokenFound,
    NoRuleEOF,
    IncorrectRule(String),
    IncorrectRules,
    WrongToken(usize,usize),
    UNIMPLEMENTED,
    EmptySource
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}