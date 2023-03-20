use std::fs;

use macro_lexer::lexer::LexerBuilder;

#[derive(Debug,Clone)]
enum TokenType{
    If,
    Else,
    Then,
    Equal,
    Id(String),
    String(String),
    EOF
}

fn main() {
    let mut builder = LexerBuilder::<TokenType>::new();
    builder.eof(TokenType::EOF)
        .rule("if", |_| TokenType::If)
        .rule("else", rule_else)
        .rule("then", rule_then)
        .rule("==", rule_equal)
        .rule("[a-zA-Z_]+", rule_id)
        .rule("\"[^\"]*\"", rule_string);
    let lexer = builder.build();
    match lexer {
        Ok(lexer) => {
            let src = "if a == b then \"hello world\" else x";

            let src = fs::read_to_string("./test.txt").unwrap();
            println!("Source size : {}",src.len());

            let tokens = lexer.run(src.as_str());

            let tokens = tokens.unwrap();
            println!("Tokens found : {}",tokens.len());
            //let f10 = tokens.iter().take(10).collect::<Vec<_>>();
            //println!("first 10 ---");
            //dbg!(f10);
        }
        Err(e) => {
            dbg!(e);
        }
    }
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
fn rule_string(s:String)->TokenType{
    TokenType::String(s)
}
fn rule_id(s:String)->TokenType{
    TokenType::Id(s)
}