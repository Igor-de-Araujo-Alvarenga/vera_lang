mod tokenizer;
mod parser;
mod codegen;
use std::io;
use crate::tokenizer::{Token};
use crate::parser::{Parser};
use crate::codegen::{generate_code};
use std::io::{Read, Write};
use std::fs::File;
use std::process::Command;

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn save_to_file(filename: &str, code: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(code.as_bytes())?;
    Ok(())
}

fn compile_with_gcc(c_file: &str, output_file: &str) -> std::io::Result<()> {
    Command::new("gcc")
        .arg(c_file)
        .arg("-o")
        .arg(output_file)
        .status()?;
    Ok(())
}
fn main() {
    let file_path ="E:/repo_estudo/vera_lang/src/test.vera";
    match read_file(file_path) {
        Ok(contents) => {
            let clean_content = contents.replace("\r","");
            let vera_lang = &clean_content;
            let tokens: Vec<Token> = Token::tokenizer(vera_lang);
            let mut parser = Parser::new(tokens.clone());
            let ast = Parser::parse(&mut parser);
            println!("{:?}", ast);
            let c_code = generate_code(&ast, &parser.symbol_table);
            println!("{:?}", c_code);
            save_to_file("./build/vera.c", &c_code).expect("Unable to save C code");
            compile_with_gcc("./build/vera.c", "./build/vera").expect("GCC compilation failed");
            println!("Compilation successful!");
            let execute_output = Command::new("./build/vera.exe")
                .output()
                .expect("Failed to execute test.exe");

            if execute_output.status.success() {
                let stdout = String::from_utf8_lossy(&execute_output.stdout);
                println!("Output: {}", stdout);
            } else {
                let stderr = String::from_utf8_lossy(&execute_output.stderr);
                println!("Error: {}", stderr);
            }

        },
        Err(e) => {
            println!("Error reading file: {}", e);
        },
    }
}


