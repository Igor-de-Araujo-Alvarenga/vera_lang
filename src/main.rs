mod tokenizer;
mod parser;
mod codegen;
use std::io;
use crate::tokenizer::{Token};
use crate::parser::{Parser};
use crate::codegen::{generate_code};
use std::io::{Read, Write};
use std::fs::{File, create_dir_all};
use std::process::Command;
use std::path::Path;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Use: vera-cli <commands> [options]");
        return;
    }
    match args[1].as_str() {
        "new" => {
            create_new_project();
        }
        _ => {
            process_commands(&args);
        }
    }
}
fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn save_to_file(filename: &str, code: &str) -> io::Result<()> {
    if let Some(parent) = Path::new(filename).parent() {
        create_dir_all(parent)?;
    }

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

fn run_file()
{
    let execute_output = Command::new("./vera.exe")
        .output()
        .expect("Failed to execute test.exe");
    if execute_output.status.success() {
        let stdout = String::from_utf8_lossy(&execute_output.stdout);
        println!("Output: {}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&execute_output.stderr);
        println!("Error: {}", stderr);
    }
}

fn create_new_project() {
    let filename = "main.vera";
    let content = "main()\n{\n}\n";
    match fs::File::create(filename) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(content.as_bytes()) {
                eprintln!("Error to create file {}: {}", filename, e);
            } else {
                println!("{} created success.", filename);
            }
        }
        Err(e) => {
            eprintln!("Error to crete file {}: {}", filename, e);
        }
    }
}

fn process_commands(args: &Vec<String>) {
    let mut input_file = String::new();
    let mut output_file = String::new();
    let mut run = false;

    let mut i = 1;
    println!("args: {:?}", args);
    while i < args.len() {
        match args[i].as_str() {
            "build" => {
                compile_file(args[i + 1].clone());
                i += 1;
            }
            "-r" | "--run" => {
                compile_file(args[i + 1].clone());
                run_file();
                run = true;
            }
            _ => {
                eprintln!("unknown command: {}", args[i]);
                return;
            }
        }
        i += 1;
    }
    if run {
        println!("Running the compiled file");
    }
}

fn compile_file(file_path: String) -> String
{
    match read_file(&file_path) {
        Ok(contents) => {
            let clean_content = contents.replace("\r","");
            let vera_lang = &clean_content;
            let tokens: Vec<Token> = Token::tokenizer(vera_lang);
            let mut parser = Parser::new(tokens.clone());
            let ast = Parser::parse(&mut parser);
            println!("{:?}", ast);
            let c_code = generate_code(&ast, &parser.symbol_table);
            println!("{:?}", c_code);
            save_to_file("./vera.c", &c_code).expect("Unable to save C code");
            compile_with_gcc("./vera.c", "./vera").expect("GCC compilation failed");
            c_code
        },
        Err(e) => {
            format!("Error reading file: {}", e)
        },
    }
}
