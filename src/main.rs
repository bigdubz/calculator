use std::cmp::PartialEq;

fn main() {
    let input = "-5*1".parse::<String>().unwrap();
    let tokens = tokenize(input);
    let answer = evaluate_expression(tokens);
    println!("{:?}", answer.value.unwrap());
}

#[derive(Debug, Copy, Clone)]
enum TokenType {
    BinOp,
    Literal,
    IntLit,
    Plus,
    Minus,
    Multi,
    MultiNeg,
    Div,
    DivNeg,
    Power,
    PowerNeg,
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
    t_type: TokenType,
    value: Option<i32>,
}

struct LitTokenBuilder {
    parent_type: TokenType,
    t_type: Option<TokenType>,
    value: Option<i32>,
}

impl LitTokenBuilder {
    pub fn new() -> LitTokenBuilder {
        LitTokenBuilder {
            parent_type: TokenType::Literal,
            t_type: Some(TokenType::IntLit),
            value: None,
        }
    }
    pub fn set_ttype(mut self, ttype: TokenType) -> LitTokenBuilder {
        self.t_type = Some(ttype);
        self
    }
    pub fn set_val(mut self, value: i32) -> LitTokenBuilder {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> Token {
        Token {
            parent_type: self.parent_type,
            t_type: self.t_type.unwrap(),
            value: self.value,
        }
    }
}

struct BinTokenBuilder {
    parent_type: TokenType,
    t_type: Option<TokenType>,
    value: Option<i32>,
}

impl BinTokenBuilder {
    pub fn new() -> BinTokenBuilder {
        BinTokenBuilder {
            parent_type: TokenType::BinOp,
            t_type: None,
            value: None,
        }
    }
    pub fn set_ttype(mut self, ttype: TokenType) -> BinTokenBuilder {
        self.t_type = Some(ttype);
        match self.t_type {
            Some(TokenType::Plus) => self.value = Some(0),
            Some(TokenType::Minus) => self.value = Some(0),
            Some(TokenType::Multi) => self.value = Some(1),
            Some(TokenType::MultiNeg) => self.value = Some(1),
            Some(TokenType::Div) => self.value = Some(1),
            Some(TokenType::DivNeg) => self.value = Some(1),
            Some(TokenType::Power) => self.value = Some(2),
            Some(TokenType::PowerNeg) => self.value = Some(2),
            _ => self.value = None,
        }
        self
    }
    pub fn build(self) -> Token {
        Token {
            parent_type: self.parent_type,
            t_type: self.t_type.unwrap(),
            value: self.value,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct BinaryExpr {
    int_lit_1: Token,
    bin_op: Token,
    int_lit_2: Token,
}

struct BinaryOperatorExpr {
    op1: Token,
    op2: Token,
}

struct OperatorLitExpr {
    op: Token,
    lit: Token,
}

impl OperatorLitExpr {
    fn evaluate_expr(&self) -> Token {
        return if self.op.t_type == TokenType::Minus {
            Token {
                parent_type: TokenType::Literal,
                t_type: self.lit.t_type,
                value: Some(-1 * self.lit.value.unwrap()),
            }
        } else if self.op.t_type == TokenType::Plus {
            self.lit
        } else if self.op.t_type == TokenType::IntLit {
            BinaryExpr {
                int_lit_1: self.op,
                bin_op: Token {
                    parent_type: TokenType::BinOp,
                    t_type: TokenType::Plus,
                    value: Some(0),
                },
                int_lit_2: self.lit,
            }
            .evaluate_expr()
        } else {
            panic!("Invalid syntax!");
        };
    }
}

impl BinaryOperatorExpr {
    fn evaluate_expr(&self) -> Token {
        return if self.op1.t_type == TokenType::Plus && self.op2.t_type == TokenType::Minus {
            BinTokenBuilder::new().set_ttype(TokenType::Minus).build()
        } else if self.op1.t_type == TokenType::Plus && self.op2.t_type == TokenType::Plus {
            BinTokenBuilder::new().set_ttype(TokenType::Plus).build()
        } else if self.op1.t_type == TokenType::Minus && self.op2.t_type == TokenType::Minus {
            BinTokenBuilder::new().set_ttype(TokenType::Plus).build()
        } else if self.op1.t_type == TokenType::Minus && self.op2.t_type == TokenType::Plus {
            BinTokenBuilder::new().set_ttype(TokenType::Minus).build()
        } else if self.op1.t_type == TokenType::Multi && self.op2.t_type == TokenType::Minus {
            BinTokenBuilder::new()
                .set_ttype(TokenType::MultiNeg)
                .build()
        } else if self.op1.t_type == TokenType::Div && self.op2.t_type == TokenType::Minus {
            BinTokenBuilder::new().set_ttype(TokenType::DivNeg).build()
        } else if self.op1.t_type == TokenType::Div && self.op2.t_type == TokenType::Plus {
            BinTokenBuilder::new().set_ttype(TokenType::Div).build()
        } else if self.op1.t_type == TokenType::Multi && self.op2.t_type == TokenType::Plus {
            BinTokenBuilder::new().set_ttype(TokenType::Multi).build()
        } else if self.op1.t_type == TokenType::MultiNeg && self.op2.t_type == TokenType::Minus {
            BinTokenBuilder::new().set_ttype(TokenType::Multi).build()
        } else if self.op1.t_type == TokenType::DivNeg && self.op2.t_type == TokenType::Minus {
            BinTokenBuilder::new().set_ttype(TokenType::Div).build()
        } else if self.op1.t_type == TokenType::Power && self.op2.t_type == TokenType::Minus {
            BinTokenBuilder::new()
                .set_ttype(TokenType::PowerNeg)
                .build()
        } else if self.op1.t_type == TokenType::Power && self.op2.t_type == TokenType::Plus {
            BinTokenBuilder::new().set_ttype(TokenType::Power).build()
        } else if self.op1.t_type == TokenType::PowerNeg && self.op2.t_type == TokenType::Minus {
            BinTokenBuilder::new().set_ttype(TokenType::Power).build()
        } else {
            BinTokenBuilder::new().set_ttype(self.op1.t_type).build()
        };
    }
}

impl BinaryExpr {
    fn evaluate_expr(&self) -> Token {
        let val1 = self.int_lit_1.value.unwrap();
        let val2 = self.int_lit_2.value.unwrap();

        return match self.bin_op.t_type {
            TokenType::Plus => LitTokenBuilder::new().set_val(val1 + val2).build(),
            TokenType::Minus => LitTokenBuilder::new().set_val(val1 - val2).build(),
            TokenType::Multi => LitTokenBuilder::new().set_val(val1 * val2).build(),
            TokenType::MultiNeg => LitTokenBuilder::new().set_val(-1 * val1 * val2).build(),
            TokenType::Div => LitTokenBuilder::new().set_val(val1 / val2).build(),
            TokenType::DivNeg => LitTokenBuilder::new().set_val(-1 * val1 / val2).build(),
            TokenType::Power => LitTokenBuilder::new()
                .set_val(val1.pow(val2 as u32))
                .build(),
            TokenType::PowerNeg => LitTokenBuilder::new()
                .set_val(1 / val1.pow(val2 as u32))
                .build(),
            _ => {
                panic!("Not implemented!")
            }
        };
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
                parent_type: TokenType::Literal,
                t_type: TokenType::IntLit,
                value: Option::from(int_lit.parse::<i32>().unwrap()),
            });
            continue;
        }

        match peek(&input_copy).as_str() {
            "+" => {
                tokens.push(Token {
                    parent_type: TokenType::BinOp,
                    t_type: TokenType::Plus,
                    value: Some(0),
                });
                input_copy = consume(&input_copy)
            }
            "-" => {
                tokens.push(Token {
                    parent_type: TokenType::BinOp,
                    t_type: TokenType::Minus,
                    value: Some(0),
                });
                input_copy = consume(&input_copy)
            }
            "*" => {
                tokens.push(Token {
                    parent_type: TokenType::BinOp,
                    t_type: TokenType::Multi,
                    value: Some(1),
                });
                input_copy = consume(&input_copy)
            }
            "/" => {
                tokens.push(Token {
                    parent_type: TokenType::BinOp,
                    t_type: TokenType::Div,
                    value: Some(1),
                });
                input_copy = consume(&input_copy)
            }
            "^" => {
                tokens.push(Token {
                    parent_type: TokenType::BinOp,
                    t_type: TokenType::Power,
                    value: Some(2),
                });
                input_copy = consume(&input_copy)
            }
            "(" => {
                tokens.push(Token {
                    parent_type: TokenType::BinOp,
                    t_type: TokenType::OpenParen,
                    value: None,
                });
                input_copy = consume(&input_copy)
            }
            ")" => {
                tokens.push(Token {
                    parent_type: TokenType::BinOp,
                    t_type: TokenType::CloseParen,
                    value: None,
                });
                input_copy = consume(&input_copy)
            }
            " " => {
                // if whitespace, consume and do nothing
                input_copy = consume(&input_copy)
            }
            _ => {
                panic!(
                    "Expected binary operator or int literal, got `{}`",
                    peek(&input_copy)
                )
            }
        }
    }

    tokens
}

fn peek(input: &str) -> String {
    let mut chars = (*input).chars();
    match chars.next() {
        None => String::new(),
        Some(c) => String::from(c),
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
    let mut precedence = 2;
    while expr_copy.len() > 1 {
        let mut see_next_precedence = true;
        for i in 0..expr_copy.len() {
            if expr_copy[i].parent_type == TokenType::BinOp
                && expr_copy[i].value.unwrap() == precedence
            {
                buffer.clear();
                if expr_copy[i + 1].parent_type == TokenType::BinOp {
                    buffer.push(expr_copy[i]);
                    buffer.push(expr_copy[i + 1]);
                    expr_copy.remove(i + 1);
                    expr_copy.remove(i);
                    expr_copy.insert(
                        i,
                        BinaryOperatorExpr {
                            op1: buffer[0],
                            op2: buffer[1],
                        }
                        .evaluate_expr(),
                    );
                    if expr_copy[i].value.unwrap() >= 1 {
                        precedence += 1;
                    }
                    buffer.clear();
                    break;
                }

                if expr_copy[i + 1].parent_type == TokenType::Literal
                    && (i as i32 - 1 < 0 || expr_copy[i - 1].parent_type != TokenType::Literal)
                {
                    buffer.push(expr_copy[i]);
                    buffer.push(expr_copy[i + 1]);
                    expr_copy.remove(i + 1);
                    expr_copy.remove(i);
                    expr_copy.insert(
                        i,
                        OperatorLitExpr {
                            op: buffer[0],
                            lit: buffer[1],
                        }
                        .evaluate_expr(),
                    );
                    buffer.clear();
                    break;
                }

                buffer.push(expr_copy[i - 1]);
                buffer.push(expr_copy[i]);
                buffer.push(expr_copy[i + 1]);
                expr_copy.remove(i + 1);
                expr_copy.remove(i);
                expr_copy.remove(i - 1);
                expr_copy.insert(
                    i - 1,
                    BinaryExpr {
                        int_lit_1: Token {
                            parent_type: TokenType::Literal,
                            t_type: TokenType::IntLit,
                            value: buffer[0].value,
                        },
                        bin_op: Token {
                            parent_type: TokenType::BinOp,
                            t_type: buffer[1].t_type,
                            value: buffer[1].value,
                        },
                        int_lit_2: Token {
                            parent_type: TokenType::Literal,
                            t_type: TokenType::IntLit,
                            value: buffer[2].value,
                        },
                    }
                    .evaluate_expr(),
                );

                see_next_precedence = false;
                break;
            }
        }
        if precedence < 0 {
            break;
        }
        if see_next_precedence {
            precedence -= 1
        }
    }
    buffer.clear();
    while expr_copy.len() != 1 {
        buffer.push(expr_copy[expr_copy.len() - 2]);
        buffer.push(expr_copy[expr_copy.len() - 1]);
        expr_copy.remove(expr_copy.len() - 1);
        expr_copy.remove(expr_copy.len() - 1);
        expr_copy.insert(
            expr_copy.len(),
            OperatorLitExpr {
                op: buffer[0],
                lit: buffer[1],
            }
            .evaluate_expr(),
        );
        buffer.clear();
    }
    expr_copy[0]
}
