use regex::{RegexSet, Regex};
use crate::utils::{lexer_error::LexerError, token::Token};


pub struct LexerBuilder<T>{
    rules : Vec<String>,
    eof : Option<T>,
    //regex_set : Option<RegexSet>,
    conversion_function:Vec<fn(String) -> T>
}

fn clean(pattern:&str) -> String {
    format!("^{}",pattern)
}
impl<T:Clone> LexerBuilder<T> {
    pub fn new() -> Self {
        Self {
            eof : None,
            rules : vec![],
            conversion_function : vec![]
        }
    }
    pub fn rule(&mut self,pattern:&str,convert:fn(String)->T) -> &mut Self {
        self.rules.push(clean(pattern));
        self.conversion_function.push(convert);
        self
    }
    pub fn eof(&mut self,token:T) -> &mut Self {
        self.eof = Some(token);
        self
    }
    pub fn build(&self)  -> Result<Lexer<T>,LexerError> {
        if self.eof.is_none() {
            return Err(LexerError::NoRuleEOF);
        }
        let eof = self.eof.clone().unwrap().clone();
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
            eof,
            regex_set:set,
            regex_vec:regexes,
            conversion_function: self.conversion_function.clone()
        })
    }
}

pub struct Lexer<T:'static> {
    eof:T,
    regex_set:RegexSet,
    regex_vec:Vec<Regex>,
    conversion_function:Vec<fn(String) -> T>
}

impl<T:Clone> Lexer<T> {
    pub fn run(&self,src:&str)->Result<Vec<Token<T>>,LexerError>{
        let mut stream:Vec<Token<T>> = vec![];

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
                    let token_type = (self.conversion_function[i])(content[.._match.end()].to_string());
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

        stream.push(Token::new(self.eof.clone(), None, None));
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