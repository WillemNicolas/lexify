type Index = usize;

#[derive(Debug)]
pub struct Token<TokenType> {
    pub token_type:TokenType,
    pub line:Option<Index>,
    pub column:Option<Index>,
}

impl<T> Token<T> {
    pub fn new(token_type:T,line:Option<Index>,column:Option<Index>)->Self{
        Token {
            token_type,
            line,
            column
        }
    }
}
