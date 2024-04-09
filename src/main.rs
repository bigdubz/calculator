use std::num::ParseIntError;
use crate::TokenType::IntLit;

fn main() {
    let input = "5 + 3".parse::<String>().unwrap();
    let tokens = tokenize(input);
    println!("{:?}", tokens);
}

#[derive(Debug)]
enum TokenType {
    IntLit,
    Plus,
    Minus,
    Multi,
    Div,
    Placeholder
}

struct Token {
    token_type: TokenType,
}

struct TokenIntLit {
    value: i32
}


fn tokenize(input: String) -> Vec<TokenType> {
    let mut tokens: Vec<TokenType> = vec![];
    let mut input_copy = String::from(input);
    let mut buffer: Vec<String> = vec![];

    while !peek(&input_copy).is_empty() {

        if peek(&input_copy).parse::<f64>().is_ok() {
            tokens.push(parse_int_lit(input_copy))

        } else if peek(&input_copy) == "+" {
            tokens.push(TokenType::Plus);
            input_copy = consume(&input_copy)
        } else if peek(&input_copy) == " " {
            input_copy = consume(&input_copy)
        }
        else { panic!("Expected `+` or `int literal`.") }
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

fn get_type(input: String) -> TokenType {
    return
        if input == "+" { TokenType::Plus }
        else if input.parse::<i32>().is_ok() { TokenType::IntLit }
        else { TokenType::Placeholder }
}

fn parse_int_lit(input: String) -> TokenIntLit {
    let mut int_lit = String::new();
    let mut input_copy = String::from(input);
    while peek(&input_copy).parse::<i32>().is_ok() {
        int_lit.push(peek(&input_copy).parse().unwrap());
        input_copy = consume(&input_copy);
    }
    
    TokenIntLit{
        value: int_lit.parse::<i32>().unwrap_or_else(|_| panic!())
    }
}