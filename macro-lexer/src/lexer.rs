use regex::Regex;

use crate::utils::tokenStream::*;
use crate::utils::lexerError::*;


#[derive(Debug)]
pub struct Lexer<TokenType>{
    rules : Vec<(String,fn(String)->TokenType)>
}

impl<TokenType> Lexer<TokenType> {

    pub fn new() -> Lexer<TokenType> {
        Lexer {
            rules : vec![]
        }
    }

    pub fn rule(&mut self, regex:&str, to:fn(String) -> TokenType) {
        let rule = (format!("^{}",regex).to_string(),to);
        self.rules.push(rule);
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
        Ok(RunnableLexer { rules })
    }
}

pub struct RunnableLexer<TokenType> {
    rules:Vec<(Regex,fn(String) -> TokenType)>

}

impl<TokenType> RunnableLexer<TokenType> {

    pub fn run(&self,src:String) -> Result<TokenStream<Token<TokenType>>,LexerError>{
        let mut stream = TokenStream::<Token<TokenType>>::new();
        let binding = src.to_owned();
        let mut content = binding.as_str();
        let blank_reg = Regex::new("^ +").unwrap();
        let mut line = 1;
        let mut column = 1;
        while content.len() > 0 {
            let mut matched = false;
            if content.chars().nth(0).unwrap() == '\n'{
                line += 1;
                column = 1;
                content = &content[1..content.len()];
                continue;
            }
            for (regex,to) in &self.rules {
                if let Some(m) = regex.find(content) {
                    let found = &content[m.start()..m.end()];
                    let token_type = (*to)(found.to_string());
                    content = &content[m.end()..content.len()];
                    stream.push(Token::new(token_type,line,column));
                    if found.contains("\n") {
                        line += 1;
                        column = found.split("\n").last().unwrap().len();
                    }else {
                        column += found.len();
                    }
                    matched = true;
                    break;
                }
            }
            if !matched {
                match blank_reg.find(content) {
                    Some(m) => {
                        content = &content[m.end()..content.len()];
                        column += m.end();
                    }
                    None => {
                        return Err(LexerError::WrongToken(line,column));
                    }
                }
            } 
        }

        Ok(stream)
    }
}