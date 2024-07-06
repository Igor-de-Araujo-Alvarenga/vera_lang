mod tokenizer;
mod parser;
use std::io;
use crate::tokenizer::{Token};
use crate::parser::{Parser};
use std::io::Read;
use std::fs::File;
fn main() {
    let file_path ="E:/repo_estudo/vera_lang/src/test.vera";
    match read_file(file_path) {
        Ok(contents) => {
            let vera_lang = &contents;
            let tokens: Vec<Token> = Token::tokenizer(vera_lang);
            let mut parser = Parser::new(tokens.clone());
            let ast = Parser::parse(&mut parser);
            println!("{:?}", tokens);
            println!("{:?}", ast);
        },
        Err(e) => {
            println!("Error reading file: {}", e);
        },
    }
}

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}