use std::mem;

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
            conversion_function: mem::take(&mut self.conversion_function),
        })
    }
}

pub struct Lexer<C> {
    eof:C,
    regex_set:RegexSet,
    regex_vec:Vec<Regex>,
    conversion_function: Vec<C>,
}

pub struct LexerWithInput<'input,'lexer,C> {
    lexer : &'lexer Lexer<C>,
    src : &'input str,
}

impl<'input,'lexer,C:Converter<'input>> Lexer<C> {
    pub fn of(&'lexer self,src:&'input str) -> LexerWithInput<'input,'lexer,C> {
        LexerWithInput { 
            lexer : &self,
            src
        }
    }
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

impl<'input,'lexer,C:Converter<'input>> IntoIterator for LexerWithInput<'input,'lexer,C> {
    type Item = Result<Token<C::T>,LexerError>;

    type IntoIter = LexerIter<'lexer,'input,C>;

    fn into_iter(self) -> Self::IntoIter {
        LexerIter::build(self.lexer,self.src)
    }
}

pub struct LexerIter<'lexer,'input,C> {
    lexer : &'lexer Lexer<C>,

    content : &'input str,
    offset : usize,

    line:usize,
    column:usize,
    error : Option<LexerError>,
    completed : bool,
}

impl<'input,'lexer,C:Converter<'input>> LexerIter<'lexer,'input,C> {
    fn build(lexer: &'lexer Lexer<C>,src:&'input str) -> Self{
        let content = src;
        let offset = 0;

        let line:usize  = 1;
        let column:usize = 1;
        Self {
            lexer,
            content,
            offset,
            line,
            column,
            error:None,
            completed : false,
        }
    }
}

impl<'input,'lexer,C:Converter<'input>> Iterator for LexerIter<'lexer,'input, C> {
    type Item = Result<Token<C::T>,LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.completed || self.error.is_some(){
            return None;
        }
        match eat_whitespace(self.content) {
            Some((new_content,o)) => {
                self.content = new_content;
                self.column += o;
                self.offset+=o;
            }
            None => {
                self.completed = true;
                return Some(Ok(Token::new(self.lexer.eof.convert(&""), None, None)));
            }
        }
        loop {
            let (new_content,o) = eat_newline(self.content);
            if o > 0 {
                self.content = new_content;
                self.offset += o;
                self.line += o;
                self.column = 1;
                continue;
            }
            break;
        }
        match self.lexer.regex_set.matches(self.content).into_iter().next(){
            Some(i) => {
                let _match = &self.lexer.regex_vec[i].find(self.content).unwrap();
                let token_type = (self.lexer.conversion_function[i]).convert(&self.content[.._match.end()]);
                self.content = &self.content[_match.end()..];
                //stream.push(Token::new(token_type, Some(line), Some(column)));
                self.column += _match.end();
                self.offset += _match.end();
                return Some(Ok(Token::new(token_type, Some(self.line), Some(self.column))));
            }
            None => {
                self.error = Some(LexerError::WrongToken(self.line, self.column));
                //return Err(LexerError::WrongToken(line, column));
                return Some(Err(LexerError::WrongToken(self.line, self.column)));
            }
        }
    }
}