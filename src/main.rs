use crate::TokenType::{IntLit, Plus};

fn main() {
    let input = "5 ^ 3".parse::<String>().unwrap();
    let tokens = tokenize(input);
    println!("{:#?}", tokens);
}

#[derive(Debug)]
enum TokenType {
    IntLit,
    Plus,
    Minus,
    Multi,
    Div,
    Power,
    OpenParen,
    CloseParen,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: Option<i32>
}


fn tokenize(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut input_copy = String::from(input);

    while !peek(&input_copy).is_empty() {

        if peek(&input_copy).parse::<i32>().is_ok() {
            // if first char is parsable into i32, keep parsing number until no longer i32, then
            // store the value inside a new Token
            let mut int_lit = String::new();
            while peek(&input_copy).parse::<i32>().is_ok() {
                int_lit.push(peek(&input_copy).parse().unwrap());
                input_copy = consume(&input_copy);
            }
            tokens.push(Token {
                token_type: IntLit,
                value: Option::from(int_lit.parse::<i32>().unwrap_or_else(|_| panic!()))
            })
        } else if peek(&input_copy) == "+" {
            // if binary operator, push the respective type and consume
            tokens.push(Token { token_type: Plus, value: None });
            input_copy = consume(&input_copy)

        } else if peek(&input_copy) == "-" {
            tokens.push(Token { token_type: TokenType::Minus, value: None });
            input_copy = consume(&input_copy)

        } else if peek(&input_copy) == "*" {
            tokens.push(Token { token_type: TokenType::Multi, value: None });
            input_copy = consume(&input_copy)

        } else if peek(&input_copy) == "/" {
            tokens.push(Token { token_type: TokenType::Div, value: None });
            input_copy = consume(&input_copy)

        } else if peek(&input_copy) == "^" {
            tokens.push(Token { token_type: TokenType::Power, value: None });
            input_copy = consume(&input_copy)

        } else if peek(&input_copy) == "(" {
            tokens.push(Token { token_type: TokenType::OpenParen, value: None });
            input_copy = consume(&input_copy)

        } else if peek(&input_copy) == ")" {
            tokens.push(Token { token_type: TokenType::CloseParen, value: None });
            input_copy = consume(&input_copy)

        }
        else if peek(&input_copy) == " " {
            // if whitespace, consume and do nothing
            input_copy = consume(&input_copy)
        }
        else { panic!("Expected binary operator or int literal, got `{}`", peek(&input_copy)) }
    }

    tokens
}

fn peek(input: &str) -> String {
    let mut chars = (*input).chars();
    match chars.next() {
        None => String::new(),
        Some(c) => String::from(c)
    }
}

fn consume(input: &str) -> String {
    let mut chars = (*input).chars();
    chars.next();
    String::from(chars.as_str())
}
