

use crate::lexer::*;

#[test]
fn test() { 
    assert!(true)
}

#[derive(Debug)]
enum TokenType{
    If,
    Else,
    Then,
    Equal,
    Id(String),
    String(String)
}


#[test]
fn test_lexer_run() {
    let mut lexer = Lexer::<TokenType>::new();
    lexer.rule("if", |_| TokenType::If);
    lexer.rule("else", |_| TokenType::Else);
    lexer.rule("then", |_| TokenType::Then);
    lexer.rule("==", |_| TokenType::Equal);
    lexer.rule("[a-z]", |s| TokenType::Id(s));
    lexer.rule("\"[a-z \\n]*\"", |s| TokenType::String(s));
    //dbg!(lexer);
    let src = String::from("   if a == b then \"hello \nword\" \nelse x");
    if let Ok(lexer) = lexer.build() {
        let res = lexer.run(src);
        assert!(res.is_ok());
        let stream = res.unwrap();
        dbg!(stream);
    }
}

use regex::Regex;

#[test]
fn test_regex() {
    let src = String::from("   if a == b then \"hello word\" else x");
    let if_reg = Regex::new(" +").unwrap();

    dbg!(if_reg.find(src.as_str()));

    let src = String::from("\"hello word\" else x");
    let string_reg = Regex::new("^\"[a-z ]*\"").unwrap();

    dbg!(string_reg.find(src.as_str()));
}