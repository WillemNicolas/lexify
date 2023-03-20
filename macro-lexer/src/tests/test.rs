
use crate::lexer_impl::*;

use crate::lexer;


#[derive(Debug,Clone)]
enum TokenType{
    If,
    Else,
    Then,
    Equal,
    NonAscii(String),
    Id(String),
    String(String),
    EOF
}

fn rule_if(_:String)->TokenType{
    TokenType::If
}
fn rule_else(_:String)->TokenType{
    TokenType::Else
}
fn rule_then(_:String)->TokenType{
    TokenType::Then
}
fn rule_equal(_:String)->TokenType{
    TokenType::Equal
}
fn rule_non_ascii(s:String)->TokenType{
    TokenType::NonAscii(s)
}

fn rule_string(s:String)->TokenType{
    TokenType::String(s)
}
fn rule_id(s:String)->TokenType{
    TokenType::Id(s)
}


#[test]
fn test_lexer_run() {
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
#[test]
fn test_lexer_non_ascii() {
    let mut builder = LexerBuilder::<TokenType>::new();
    builder.eof(TokenType::EOF)
        .rule("if", rule_if)
        .rule("else", rule_else)
        .rule("then", rule_then)
        .rule("==", rule_equal)
        .rule("國", rule_non_ascii)
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

