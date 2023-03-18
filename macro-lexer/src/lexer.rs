use regex::Regex;

use crate::utils::token::*;
use crate::utils::lexer_error::*;

use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Lexer<TokenType>{
    rules : Vec<(String,fn(String)->TokenType)>,
    rule_eof : Option<fn()->TokenType>
}

impl<TokenType> Lexer<TokenType> {

    pub fn new() -> Lexer<TokenType> {
        Lexer {
            rules : vec![],
            rule_eof : None
        }
    }

    pub fn rule(&mut self, regex:&str, to:fn(String) -> TokenType) {
        let rule = (format!("^{}",regex).to_string(),to);
        self.rules.push(rule);
    }
    pub fn rule_eof(&mut self,to:fn() -> TokenType) {
        self.rule_eof = Some(to);
    }
    pub fn build(&self) -> Result<RunnableLexer<TokenType>,LexerError >{
        if self.rules.len() == 0 {
            return Err(LexerError::IncorrectRule(String::from("No rule defined")));
        }

        let mut rules:Vec<(Regex,fn(String) -> TokenType)> = vec![];

        for (pattern,to) in &self.rules {
            let reg = Regex::new(pattern.as_str());
            match reg {
                Ok(r) => {
                    rules.push((r,*to))
                }
                Err(e) => {
                    return Err(LexerError::IncorrectRule(e.to_string()))
                }
            }
        }
        match self.rule_eof {
            Some(rule_eof) => {
                return Ok(RunnableLexer { rules,rule_eof });
            }
            None => {
                return Err(LexerError::NoRuleEOF)
            }
        }
        
    }
}


fn new_line_offset(line:usize,offset:usize) -> (usize,usize) {
    (line+1,offset)
}
fn new_line(line:usize) -> (usize,usize) {
    (line+1,1)
}
fn update_line_column(token:&str,line:usize,column:usize)->(usize,usize){
    if token.contains('\n') {
        let offset = token.split('\n').last().unwrap().len();
        return new_line_offset(line, offset);
    }
    return (line,column+token.len());
}

pub struct RunnableLexer<TokenType> {
    rules:Vec<(Regex,fn(String) -> TokenType)>,
    rule_eof:fn() -> TokenType
}

impl<TokenType> RunnableLexer<TokenType> {
    


    pub fn run(&self,src:String) -> Result<Vec<Token<TokenType>>,LexerError>{

        let mut stream = Vec::new();
        let binding = src.to_owned();
        let content = binding.as_str();
        let mut cursor:usize = 0;
        let size = content.len();
        lazy_static! {
            static ref BLANK_REGEX:Regex = Regex::new("^ +").unwrap();
        }
        let mut line = 1;
        let mut column = 1;


        while cursor < size {
            let first_char = content.chars().nth(cursor).unwrap();
            if first_char == '\n'{
                (line,column) = new_line(line);
            }

            let sub = &content[cursor..];

            let first_match = self.rules.iter()
                .find_map(|(regex,to)| 
                    match regex.find(sub) {
                        Some(m) => Some((m,*to)),
                        None => None
                    }   
                );

            match first_match {
                Some((_match,to))=>{
                    let found = &content[cursor .. cursor+_match.end()];
                    cursor += _match.end();
                    let token_type = (to)(found.to_string());
                    stream.push(Token::new(token_type,Some(line),Some(column)));
                    (line,column) = update_line_column(found,line,column);
                },
                None => {
                    if first_char == '\n' {
                        cursor+=1;
                        continue;
                    }
                    match BLANK_REGEX.find(sub) {
                        Some(_match) => {
                            cursor += _match.end();
                            column += _match.end();
                        }
                        None => {
                            return Err(LexerError::WrongToken(line,column));
                        }
                    }
                }
            };
        }
        stream.push(Token::new((self.rule_eof)(),None,None));
        Ok(stream)
    }
}