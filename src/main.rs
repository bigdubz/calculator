use std::cmp::PartialEq;


fn main() {
    let input = "2^3".parse::<String>().unwrap();
    let tokens = tokenize(input);
    let answer = evaluate_expression(tokens);
    println!("{:?}", answer);
}

#[derive(Debug, Copy, Clone)]
enum TokenType {
    BinaryOperator,
    Literal,
    IntLit,
    Plus,
    Minus,
    Multi,
    Div,
    Power,
    OpenParen,
    CloseParen,
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

#[derive(Debug, Copy, Clone)]
struct Token {
    parent_type: TokenType,
    token_type: TokenType,
    value: Option<i32>
}

#[derive(Debug, Copy, Clone)]
struct BinaryExpression {
    int_lit_1: Token,
    bin_op: Token,
    int_lit_2: Token
}

impl BinaryExpression {
    fn evaluate_expr(&self) -> Token {
        let val1 = self.int_lit_1.value.unwrap_or_else(|| { panic!() });
        let val2 = self.int_lit_2.value.unwrap_or_else(|| { panic!() });
        return 
        if self.bin_op.token_type == TokenType::Plus {
            Token { parent_type: TokenType::Literal, token_type: TokenType::IntLit, value: Some(val1 + val2) }
        } else if self.bin_op.token_type == TokenType::Minus {
            Token { parent_type: TokenType::Literal, token_type: TokenType::IntLit, value: Some(val1 - val2) }
        } else if self.bin_op.token_type == TokenType::Multi {
            Token { parent_type: TokenType::Literal, token_type: TokenType::IntLit, value: Some(val1 * val2) }
        } else if self.bin_op.token_type == TokenType::Div {
            Token { parent_type: TokenType::Literal, token_type: TokenType::IntLit, value: Some(val1 / val2) }
        } else if self.bin_op.token_type == TokenType::Power {
            Token { parent_type: TokenType::Literal, token_type: TokenType::IntLit, value: Some(val1.pow(val2 as u32)) }
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
            tokens.push(Token { parent_type: TokenType::Literal, token_type: TokenType::IntLit, value:
                 Option::from(int_lit.parse::<i32>().unwrap_or_else(|_| panic!())) })
        } else if peek(&input_copy) == "+" {
            // if binary operator, push the respective type and consume
            tokens.push(Token { parent_type: TokenType::BinaryOperator, token_type: TokenType::Plus, value: Some(0) });
            input_copy = consume(&input_copy)

        } else if peek(&input_copy) == "-" {
            tokens.push(Token { parent_type: TokenType::BinaryOperator, token_type: TokenType::Minus, value: Some(0) });
            input_copy = consume(&input_copy)

        } else if peek(&input_copy) == "*" {
            tokens.push(Token { parent_type: TokenType::BinaryOperator, token_type: TokenType::Multi, value: Some(1) });
            input_copy = consume(&input_copy)

        } else if peek(&input_copy) == "/" {
            tokens.push(Token { parent_type: TokenType::BinaryOperator, token_type: TokenType::Div, value: Some(1) });
            input_copy = consume(&input_copy)

        } else if peek(&input_copy) == "^" {
            tokens.push(Token { parent_type: TokenType::BinaryOperator, token_type: TokenType::Power, value: Some(2) });
            input_copy = consume(&input_copy)

        } else if peek(&input_copy) == "(" {
            tokens.push(Token { parent_type: TokenType::BinaryOperator, token_type: TokenType::OpenParen, value: None });
            input_copy = consume(&input_copy)

        } else if peek(&input_copy) == ")" {
            tokens.push(Token { parent_type: TokenType::BinaryOperator, token_type: TokenType::CloseParen, value: None });
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
    let mut expr_copy: Vec<Token> = Vec::from(expr);
    let mut buffer: Vec<Token> = Vec::new();
    let mut precedence = 3;
    while expr_copy.len() != 1 {
        let mut see_next_precedence = true;
        for i in 0..expr_copy.len() {
            if expr_copy[i].parent_type == TokenType::BinaryOperator && 
                expr_copy[i].value.unwrap_or_else(|| { panic!() }) == precedence {
                buffer.push(expr_copy[i - 1]);
                buffer.push(expr_copy[i]);
                buffer.push(expr_copy[i + 1]);
                expr_copy.remove(i + 1);
                expr_copy.remove(i);
                expr_copy.remove(i - 1);
                expr_copy.insert(i-1, BinaryExpression {
                        int_lit_1: Token {
                            parent_type: TokenType::Literal,
                            token_type: TokenType::IntLit,
                            value: buffer[0].value
                        },
                        bin_op: Token {
                            parent_type: TokenType::BinaryOperator, 
                            token_type: buffer[1].token_type,
                            value: buffer[1].value
                        },
                        int_lit_2: Token {
                            parent_type: TokenType::Literal,
                            token_type: TokenType::IntLit,
                            value: buffer[2].value
                        }
                    }
                    .evaluate_expr()
                );
                see_next_precedence = false;
                buffer.clear();
                break;
            }
        }
        if see_next_precedence { precedence -= 1 }
        if precedence < 0 { break }
    }
    expr_copy[0]
}