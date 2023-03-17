use super::lexerError::*;

type Index = usize;

#[derive(Debug)]
pub struct Token<TokenType> {
    token_type:TokenType,
    line:Index,
    column:Index,
}

impl<T> Token<T> {
    pub fn new(token_type:T,line:Index,column:Index)->Self{
        Token {
            token_type,
            line,
            column
        }
    }
}

#[derive(Debug)]
pub struct TokenStream<Token> {
    tokens:Vec<Token>,
    cursor:Index
}

impl<T> TokenStream<Token<T>> {
    pub fn new()-> TokenStream<Token<T>> {
        TokenStream {
            tokens : vec![],
            cursor : 0
        }
    }   
    pub fn push(&mut self,token:Token<T>){
        self.tokens.push(token);
    }
    fn next(&mut self) -> Option<&Token<T>> {
        if self.tokens.len() == 0 {
            return None;
        }
        let current = if self.cursor < self.tokens.len() {
            Some(&self.tokens[self.cursor])
        } else {
            None
        };
        self.cursor += 1;
        return current;
    }
}