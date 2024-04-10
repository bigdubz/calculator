use crate::TokenType::{IntLit, Plus};

fn main() {
    let input = "27/3 * 5 +13-5".parse::<String>().unwrap();
    let tokens = tokenize(input);
    let answer = evaluate_expression(tokens);
    println!("{:?}", answer);
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
struct Token {
    token_type: TokenType,
    value: Option<i32>
}


struct BinaryExpression {
    int_lit_1: Token,
    bin_op: Token,
    int_lit_2: Token
}

impl BinaryExpression {
    fn evaluate_expr(&self) -> Token {
        let val1 = self.int_lit_1.value.unwrap_or_else(|| 0);
        let val2 = self.int_lit_2.value.unwrap_or_else(|| 0);
        return 
        if std::mem::discriminant(&self.bin_op.token_type) ==
            std::mem::discriminant(&Plus) {
            Token { token_type: IntLit, value: Some(val1 + val2) }
        } else if std::mem::discriminant(&self.bin_op.token_type) ==
            std::mem::discriminant(&TokenType::Minus) {
            Token { token_type: IntLit, value: Some(val1 - val2) }
        } else if std::mem::discriminant(&self.bin_op.token_type) ==
            std::mem::discriminant(&TokenType::Multi) {
            Token { token_type: IntLit, value: Some(val1 * val2) }
        } else if std::mem::discriminant(&self.bin_op.token_type) ==
            std::mem::discriminant(&TokenType::Div) {
            Token { token_type: IntLit, value: Some(val1 / val2) }
        } else { panic!("Not implemented") }
    }
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

        } else if peek(&input_copy) == " " {
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

fn evaluate_expression(expr: Vec<Token>) -> Token {
    let mut expr_copy = Vec::from(expr);
    let mut buffer: Vec<Token> = Vec::new();
    while expr_copy.len() != 1 {
        loop {
            buffer.push(expr_copy[0]);
            expr_copy.remove(0);
            if buffer.len() == 3 {
                if std::mem::discriminant(&buffer[0].token_type) == std::mem::discriminant(&TokenType::IntLit) &&
                   (std::mem::discriminant(&buffer[1].token_type) == std::mem::discriminant(&TokenType::Plus) ||
                   std::mem::discriminant(&buffer[1].token_type) == std::mem::discriminant(&TokenType::Minus) ||
                   std::mem::discriminant(&buffer[1].token_type) == std::mem::discriminant(&TokenType::Multi) ||
                   std::mem::discriminant(&buffer[1].token_type) == std::mem::discriminant(&TokenType::Div) ||
                   std::mem::discriminant(&buffer[1].token_type) == std::mem::discriminant(&TokenType::Power)) &&
                   std::mem::discriminant(&buffer[2].token_type) == std::mem::discriminant(&TokenType::IntLit)  {
                    break
                }
            }
        }
        let a = BinaryExpression {
            int_lit_1: Token {
                token_type: IntLit,
                value: buffer[0].value
            },
            bin_op: Token {
                token_type: buffer[1].token_type,
                value: None
            },
            int_lit_2: Token {
                token_type: IntLit,
                value: buffer[2].value
            }
        }.evaluate_expr();
        expr_copy.insert(0, a);
        buffer.clear();
    }

    expr_copy[0]
}