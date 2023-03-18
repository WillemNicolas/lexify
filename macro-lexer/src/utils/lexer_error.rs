#[derive(Debug)]
pub enum LexerError {
    NoTokenFound,
    NoRuleEOF,
    IncorrectRule(String),
    WrongToken(usize,usize)
}
