
use crate::lexer_impl::*;

use crate::lexer;

/*
#[derive(Debug,Clone)]
enum TokenType<'input>{
    If,
    Else,
    Then,
    Equal,
    Id(&'input str),
    String(&'input str),
    EOF
}

fn rule_if<'input>(s:&'input str)->TokenType{
    TokenType::If
}
fn rule_else<'input>(s:&'input str)->TokenType{
    TokenType::Else
}
fn rule_then<'input>(s:&'input str)->TokenType{
    TokenType::Then
}
fn rule_equal<'input>(s:&'input str)->TokenType{
    TokenType::Equal
}

fn rule_string<'input>(s:&'input str)->TokenType{
    TokenType::String(s)
}
fn rule_id<'input>(s:&'input str)->TokenType{
    TokenType::Id(s)
}
*/
#[derive(Debug)]
enum TokenType<'input>{
    If,
    Else,
    Then,
    Equal,
    Id(&'input str),
    String(&'input str),
    EOF
}

#[derive(Debug)]
enum Token{
    If,
    Else,
    Then,
    Equal,
    Id,
    String,
    EOF
}
impl<'input> Converter<'input> for Token {
    type T = TokenType<'input>;
    fn convert(&self, src:&'input str) -> Self::T {
        match self {
            Token::If => TokenType::If,
            Token::Else => TokenType::Else,
            Token::Then => TokenType::Then,
            Token::Equal => TokenType::Equal,
            Token::Id => TokenType::Id(src),
            Token::String => TokenType::String(src),
            Token::EOF => TokenType::EOF,
        }
    }
}

#[test]
fn test_lexer_run() {
    let mut builder = LexerBuilder::<Token>::new();
    builder.eof(Token::EOF)
        .rule("if", Token::If)
        .rule("else", Token::Else)
        .rule("then", Token::Then)
        .rule("==", Token::Equal)
        .rule("[a-zA-Z_]+", Token::Id)
        .rule("\"[^\"]*\"", Token::String);
    let lexer = builder.build();
    match lexer {
        Ok(lexer) => {
            let src = " if a == b then \"hello world\" else x";
            let tokens = lexer.run(src);

            assert!(tokens.is_ok());
            let tokens = tokens.unwrap();
            dbg!(&tokens);
            assert!(tokens.len() == 9);
        }
        Err(e) => {
            dbg!(e);
        }
    }
}

/*
#[test]
fn test_lexer_non_ascii() {
    let mut builder = LexerBuilder::<TokenType>::new();
    builder.eof(TokenType::EOF)
        .rule("if", rule_if)
        .rule("else", rule_else)
        .rule("then", rule_then)
        .rule("==", rule_equal)
        .rule("[a-zA-Z_]+", rule_id)
        .rule("\"[^\"]*\"", rule_string);
    let lexer = builder.build();
    match lexer {
        Ok(lexer) => {
            let src = "  國  if a == b then \"hello world\" else x";
            let tokens = lexer.run(src);

            assert!(tokens.is_ok());
            let tokens = tokens.unwrap();
            assert!(tokens.len() == 10);
        }
        Err(e) => {
            dbg!(e);
        }
    }
}

#[test]
fn test_lexer_line_number() {
    let mut builder = LexerBuilder::<TokenType>::new();
    builder.eof(TokenType::EOF)
        .rule("if", rule_if)
        .rule("else", rule_else)
        .rule("then", rule_then)
        .rule("==", rule_equal)
        .rule("[a-zA-Z_]+", rule_id)
        .rule("\"[^\"]*\"", rule_string);
    let lexer = builder.build();
    match lexer {
        Ok(lexer) => {
            let src = " if a == b then\n \"hello world\"\n else x";
            let tokens = lexer.run(src);

            assert!(tokens.is_ok());
            let tokens = tokens.unwrap();
            let number_lines = tokens.iter()
                .map(|t| 
                    t.line.unwrap_or(0))
                .max()
                .unwrap();
            assert!(number_lines == 3);
        }
        Err(e) => {
            dbg!(e);
        }
    }
}

#[test]
fn test_macro_lexer() {
    let lexer = lexer!{
        with TokenType,
        rule "[a-zA-Z_]+" => |s| {TokenType::Id(s)},
        rule "if" => {TokenType::If},
        rule "else" => {TokenType::Else},
        rule "then" => {TokenType::Then},
        rule "==" => || {TokenType::Equal},
        rule "[a-zA-Z_]+" => |s| {TokenType::Id(s)},
        rule "\"[^\"]*\"" => |s| {TokenType::String(s)},
        eof TokenType::EOF,
    };
    match lexer {
        Ok(lexer) => {
            let src = " if a == b then\n \"hello world\"\n else x";
            let tokens = lexer.run(src);

            assert!(tokens.is_ok());
            let tokens = tokens.unwrap();
            let number_lines = tokens.iter()
                .map(|t| 
                    t.line.unwrap_or(0))
                .max()
                .unwrap();
            assert!(number_lines == 3);
        }
        Err(e) => {
            dbg!(e);
        }
    }
}
*/