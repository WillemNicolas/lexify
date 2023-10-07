use std::{mem};

use regex::{RegexSet, Regex};
use crate::utils::{lexer_error::LexerError, token::Token};


pub trait Converter<'input>{
    type T;
    fn convert(&self,src:&'input str) -> Self::T;
}

pub struct LexerBuilder<C> {
    rules : Vec<String>,
    eof : Option<C>,
    conversion_function: Vec<C>,
}

fn clean(pattern:&str) -> String {
    format!("^{}",pattern)
}
impl<C :for<'input> Converter<'input>> LexerBuilder<C> {
    pub fn new() -> Self {
        Self {
            eof : None,
            rules : vec![],
            conversion_function : vec![]
        }
    }
    pub fn rule(&mut self,pattern:&str,convert:C) -> &mut Self {
        self.rules.push(clean(pattern));
        self.conversion_function.push(convert);
        self
    }
    pub fn eof(&mut self,eof_converter:C) -> &mut Self {
        self.eof = Some(eof_converter);
        self
    }
    pub fn build(&mut self)  -> Result<Lexer<C>,LexerError> {
        if self.eof.is_none() {
            return Err(LexerError::NoRuleEOF);
        }
        if self.rules.is_empty() || self.rules.len() != self.conversion_function.len() {
            return Err(LexerError::IncorrectRules);
        }
        let set = RegexSet::new(&self.rules);
        if let Err(e) = set {
            return Err(LexerError::IncorrectRule(e.to_string()));
        }

        let set = set.unwrap().clone();
        let regexes: Vec<Regex> = set.patterns().iter()
            .map(|pat| Regex::new(pat).unwrap())
            .collect();
        Ok(Lexer {  
            eof: mem::take(&mut self.eof).unwrap(),
            regex_set:set,
            regex_vec:regexes,
            conversion_function: mem::take(&mut self.conversion_function)
        })
    }
}

pub struct Lexer<C> {
    eof:C,
    regex_set:RegexSet,
    regex_vec:Vec<Regex>,
    conversion_function: Vec<C>
}

impl<'input,C:Converter<'input>> Lexer<C> {
    pub fn run(&self,src:&'input str)->Result<Vec<Token<C::T>>,LexerError>{
        let mut stream:Vec<Token<C::T>> = Vec::new();

        let src_size = src.len();

        let mut content = src;
        let mut offset = 0;

        let mut line:usize  = 1;
        let mut column:usize = 1;

        while  offset < src_size {
            match eat_whitespace(content) {
                Some((new_content,o)) => {
                    content = new_content;
                    column += o;
                    offset+=o;
                }
                None => {
                    break;
                }
            }

            let (new_content,o) = eat_newline(content);
            if o > 0 {
                content = new_content;
                offset += o;
                line += o;
                column = 1;
                continue;
            }

            match self.regex_set.matches(content).into_iter().next(){
                Some(i) => {
                    let _match = &self.regex_vec[i].find(content).unwrap();
                    let token_type = (self.conversion_function[i]).convert(&content[.._match.end()]);
                    content = &content[_match.end()..];
                    stream.push(Token::new(token_type, Some(line), Some(column)));
                    column += _match.end();
                    offset += _match.end();
                }
                None => {
                    return Err(LexerError::WrongToken(line, column));
                }
            }
        }

        stream.push(Token::new(self.eof.convert(&""), None, None));
        Ok(stream)
    }
}

fn eat_whitespace(src:&str)->Option<(&str,usize)>{
    match src.char_indices().find(|(_,c)| *c != ' ') {
        Some((i,_)) => Some((&src[i..],i)),
        None => None
    }
}

fn eat_newline(src:&str)-> (&str,usize) {
    let new_line_offset = src.char_indices().take_while(|(_,c)| *c == '\n').count();
    return (&src[new_line_offset..],new_line_offset)
}