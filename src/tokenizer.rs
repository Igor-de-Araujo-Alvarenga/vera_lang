use core::str::Chars;
use std::collections::HashMap;
use std::iter::Peekable;
use std::string;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
    Number(String),
    Main,
    Print,
    RBrace,
    LBrace,
    LineBreak,
    If,
    Else,
    ElseIf,
    Identifier(String),
    StringType,
    IntegerType,
    BooleanType,
    Assignment,
    LessThan,
    LessEqualThan,
    BiggerThan,
    BiggerEqualThan,
    EqualThan,
    DifferentThan,
    StringLiteral(String),
}
impl Token {
    pub fn tokenizer(input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();
        while let Some(&ch) = chars.peek() {
            match ch {
                '0'..='9' => {
                    Token::tokenizer_numbers(&mut tokens, &mut chars);
                }
                '+' | '-' | '*' | '/' => {
                    Token::tokenizer_math_operators(&mut tokens, &mut chars);
                }
                '(' | ')' | '{' | '}' | ' ' | '\t' | '\n' | '=' | '<' | '>' | '!' => {
                    Token::tokenizer_symbols(&mut tokens, &mut chars);
                }
                '"' => {
                    Token::tokenizer_string_literal(&mut tokens, &mut chars);
                }
                _ if ch.is_alphabetic() => {
                    Token::tokenizer_keywords(&mut tokens, &mut chars);
                }
                _ => panic!("Unexpected character: {}", ch),
            }
        }
        println!("{:?}", tokens);
        tokens
    }

    pub fn tokenizer_numbers(tokens: &mut Vec<Token>, chars: &mut Peekable<Chars>) {
        let mut num = String::new();
        while let Some(&ch) = chars.peek() {
            if ch.is_digit(10) {
                num.push(ch);
                chars.next();
            } else {
                break;
            }
        }
        tokens.push(Token::Number(num));
    }

    pub fn tokenizer_math_operators(tokens: &mut Vec<Token>, chars: &mut Peekable<Chars>) {
        if let Some(ch) = chars.next() {
            match ch {
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Multiply),
                '/' => tokens.push(Token::Divide),
                _ => (),
            }
        }
    }

    pub fn tokenizer_symbols(tokens: &mut Vec<Token>, chars: &mut Peekable<Chars>) {
        if let Some(ch) = chars.next() {
            match ch {
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),
                '{' => tokens.push(Token::LBrace),
                '}' => tokens.push(Token::RBrace),
                ' ' | '\t' | '\n' => {},
                '=' => {
                    let next_char = chars.peek().unwrap();
                    if ch == '=' && next_char.to_owned() == '=' {
                        chars.next();
                        tokens.push(Token::EqualThan);
                    } else {
                        tokens.push(Token::Assignment);
                    }
                }
                '>' => {
                    let next_char = chars.peek().unwrap();
                    if ch == '>' && next_char.to_owned() == '=' {
                        chars.next();
                        tokens.push(Token::BiggerEqualThan);
                    } else {
                        tokens.push(Token::BiggerThan);
                    }
                }
                '<' => {
                    let next_char = chars.peek().unwrap();
                    if ch == '<' && next_char.to_owned() == '=' {
                        chars.next();
                        tokens.push(Token::LessEqualThan);
                    } else {
                        tokens.push(Token::LessThan);
                    }
                }
                '!' => {
                    let next_char = chars.peek().unwrap();
                    if ch == '!' && next_char.to_owned() == '=' {
                        chars.next();
                        tokens.push(Token::DifferentThan);
                    }
                }
                _ => (),
            }
        }
    }

    fn tokenizer_keywords(
        tokens: &mut Vec<Token>,
        chars: &mut std::iter::Peekable<std::str::Chars>,
    ) {
        let mut ident = String::new();
        while let Some(&ch) = chars.peek() {
            if ch.is_alphanumeric() {
                ident.push(ch);
                chars.next();
            } else {
                break;
            }
        }
        match ident.as_str() {
            "main" => tokens.push(Token::Main),
            "if" => tokens.push(Token::If),
            "elseif" => tokens.push(Token::ElseIf),
            "else" => tokens.push(Token::Else),
            "string" => tokens.push(Token::StringType),
            "integer" => tokens.push(Token::IntegerType),
            "boolean" => tokens.push(Token::BooleanType),
            "print" => tokens.push(Token::Print),
            _ => tokens.push(Token::Identifier(ident)),
        }
    }

    fn tokenizer_string_literal(tokens: &mut Vec<Token>, chars: &mut Peekable<Chars>) {
        let mut string_literal = String::new();
        chars.next();
        while let Some(&ch) = chars.peek() {
            if ch == '"' {
                chars.next();
                break;
            } else {
                string_literal.push(ch);
                chars.next();
            }
        }
        tokens.push(Token::StringLiteral(string_literal));
    }
}
