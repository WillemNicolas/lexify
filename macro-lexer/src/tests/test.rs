
use crate::lexer::*;

use crate::lexer;


#[derive(Debug)]
enum TokenType{
    If,
    Else,
    Then,
    Equal,
    Id(String),
    String(String),
    EOF
}


#[test]
fn test_lexer_run() {
    let mut lexer = Lexer::<TokenType>::new();
    lexer.rule("[a-z]", |s| TokenType::Id(s));
    lexer.rule("if", |_| TokenType::If);
    lexer.rule("else", |_| TokenType::Else);
    lexer.rule("then", |_| TokenType::Then);
    lexer.rule("==", |_| TokenType::Equal);
    lexer.rule("\"[a-z \\n]*\"", |s| TokenType::String(s));
    lexer.rule_eof(|| TokenType::EOF);
    //dbg!(lexer);
    let src = String::from("   ifo a == b then \"hello \nword\" \nelse x");

    if let Ok(lexer) = lexer.build() {
        let res = lexer.run(src);
        assert!(res.is_ok());
        let stream = res.unwrap();
        dbg!(stream);
    }
}

#[test]
fn test_macro_lexer() {
    lexer!();
}