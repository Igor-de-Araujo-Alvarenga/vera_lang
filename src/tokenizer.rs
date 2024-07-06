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
    LBrace
}
impl Token {
    pub fn tokenizer(input: &str) -> Vec<Token>
    {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(&ch) = chars.peek() {
            match ch {
                '0'..= '9' => {
                    let mut num = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_digit(10){
                            num.push(ch);
                            chars.next();
                        }else {
                            break;
                        }
                    }
                    tokens.push(Token::Number(num));
                }
                '+' => {
                    tokens.push(Token::Plus);
                    chars.next();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    chars.next();
                }
                '*' => {
                    tokens.push(Token::Multiply);
                    chars.next();
                }
                '/' => {
                    tokens.push(Token::Divide);
                    chars.next();
                }
                '(' => {
                    tokens.push(Token::LParen);
                    chars.next();
                }
                ')' => {
                    tokens.push(Token::RParen);
                    chars.next();
                }
                '}' => {
                    tokens.push(Token::RBrace);
                    chars.next();
                }
                '{' => {
                    tokens.push(Token::LBrace);
                    chars.next();
                }
                ' ' | '\t' | '\n' => {
                    chars.next();
                }
                'm' => {
                    let mut ident = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_alphabetic() {
                            ident.push(ch);
                            chars.next();
                        }else {
                            break;
                        }
                    }
                    if ident == "main"{
                        tokens.push(Token::Main);
                    }else {
                        panic!("Unexpected identifier: {}", ident);
                    }

                }
                _ => panic!("Unexpected character: {}", ch),
            }
        }
        tokens
    }
}
