use std::fs;

use macro_lexer::lexer_impl::LexerBuilder;
use macro_lexer::lexer;

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

    let lexer = lexer!{
        with TokenType,
        rule "if" => {TokenType::If},
        rule "else" => {TokenType::Else},
        rule "then" => {TokenType::Then},
        rule "==" => {TokenType::Equal},
        rule "[a-zA-Z_]+" => |s| {TokenType::Id(s)},
        rule "\"[^\"]*\"" => |s| {TokenType::String(s)},
        eof TokenType::EOF,
    };
    match lexer {
        Ok(lexer) => {
            let src = " if a == b then\n \"hello world\"\n else x";
            //let src = fs::read_to_string("./test.txt").unwrap();
            //let src = src.as_str();
            println!("Source size : {}",src.len());
            let tokens = lexer.run(src);
            let tokens = tokens.unwrap();
            println!("Tokens found : {}",tokens.len());
        }
        Err(e) => {
            dbg!(e);
        }
    }

    /*let mut builder = LexerBuilder::<TokenType>::new();
    builder.eof(TokenType::EOF)
        .rule("if", |_| TokenType::If)
        .rule("else", rule_else)
        .rule("then", rule_then)
        .rule("==", rule_equal)
        .rule("[a-zA-Z_]+", rule_id)
        .rule(r#""((\u{5c}\u{22})|[^\u{5c}\u{22}])*""#, rule_string);
    let lexer = builder.build();
    match lexer {
        Ok(lexer) => {
            let src = "if a == b then \"hello\\\" world\" else x";

            //let src = fs::read_to_string("./test.txt").unwrap();
            //let src = src.as_str();
            println!("Source size : {}",src.len());

            let tokens = lexer.run(src);

            let tokens = tokens.unwrap();
            println!("Tokens found : {}",tokens.len());
            //let f10 = tokens.iter().take(10).collect::<Vec<_>>();
            //println!("first 10 ---");
            //dbg!(f10);
        }
        Err(e) => {
            dbg!(e);
        }
    }*/
}
