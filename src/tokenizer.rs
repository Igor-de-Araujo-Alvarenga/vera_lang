use std::iter::Peekable;
use core::str::Chars;
use crate::tokenizer;

#[derive(Debug, PartialEq, Clone)]
pub enum Token
{
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
    Number(String),
    Main,
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
    Assignment
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
                '(' | ')' | '{' | '}' | ' ' | '\t' | '\n' | '=' => {
                    Token::tokenizer_symbols(&mut tokens, &mut chars);
                }
                _ if ch.is_alphabetic() => {
                    Token::tokenizer_identifiers(&mut tokens, &mut chars);
                }
                'm' => {
                    let mut ident = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_alphabetic() {
                            ident.push(ch);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    if ident == "main" {
                        tokens.push(Token::Main);
                    } else {
                        panic!("Unexpected identifier: {}", ident);
                    }
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
                ' ' | '\t' => {}
                '\n' => tokens.push(Token::LineBreak),
                '=' => tokens.push(Token::Assignment),
                _ => (),
            }
        }
    }

    fn tokenizer_identifiers(tokens: &mut Vec<Token>, chars: &mut std::iter::Peekable<std::str::Chars>) {
        let mut ident = String::new();
        while let Some(&ch) = chars.peek() {
            if ch.is_alphabetic() {
                ident.push(ch);
                chars.next();
            } else {
                break;
            }
        }
        match ident.as_str() {
            "main" => tokens.push(Token::Main),
            "if" => tokens.push(Token::If),
            "else" => tokens.push(Token::Else),
            "string" => tokens.push(Token::StringType),
            "integer" => tokens.push(Token::IntegerType),
            "boolean" => tokens.push(Token::BooleanType),
            _ => tokens.push(Token::Identifier(ident)),
        }
    }

}
