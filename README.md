# macro-lexer
This is a lexer library for Rust programming language. It provides a way to tokenize input source code into a sequence of tokens using regular expressions. 
The library is written in Rust and can be used by other Rust projects as a dependency.

# Installation
Add the following to your Cargo.toml file:
```
[dependencies]
macro-lexer = "0.1.0"
```

# Usage
To use the macro-lexer library, you need to build a lexer with the provided macro and run it with the source code you want to tokenize. Here's an example:
```
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
                let src = fs::read_to_string("./test.txt").unwrap();
                let src = src.as_str();
                let tokens = lexer.run(src);
                let tokens = tokens.unwrap();
                dbg!(tokens);
            }
            Err(e) => {
                dbg!(e);
            }
    }
}
```

the macro lexer uses a dsl to build the lexer with the simple grammar :
```
    with Type,
    rule String => fn(String) -> Type, 
    eof Type::Value,
```



# License
The macro-lexer library is licensed under the MIT License. See the LICENSE file for details.
