use std::fs;
use lexify::lexer_impl::{LexerBuilder, Converter};
use lexify::lexer;

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

fn main() {

    let lexer = lexer!{
        with Token,
        rule "if" => Token::If,
        rule "else" => Token::Else,
        rule "then" => Token::Then,
        rule "==" => Token::Equal,
        rule "[a-zA-Z_]+" => Token::Id,
        rule "\"[^\"]*\"" => Token::String,
        eof Token::EOF,
    };
    match lexer {
        Ok(lexer) => {
            //let src = " if a == b then\n \"hello world\"\n else x";
            let src = fs::read_to_string("./test.txt").unwrap();
            let src = src.as_str();
            //println!("Source size : {}",src.len());
            //let tokens = lexer.run(src);
            //let tokens = tokens.unwrap();
            let mut tokens_len = 0;
            //tokens_len = tokens.len();s
            for _ in lexer.of(src) {
            //for _ in src.to_string().chars() {
                tokens_len += 1;
                //dbg!(&token);
            }
            println!("Tokens found : {}",tokens_len);
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
