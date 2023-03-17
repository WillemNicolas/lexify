#[derive(Debug)]
pub enum LexerError {
    NoTokenFound,
    IncorrectRule(String),
    WrongToken(usize,usize)
}
