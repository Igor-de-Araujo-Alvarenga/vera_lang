mod parser;
mod ast;
mod semantic;
mod codegen;

use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <source file>", args[0]);
        return;
    }

    let filename = &args[1];
    let source_code = fs::read_to_string(filename).expect("Unable to read file");

    match parser::parse_program(&source_code) {
        Ok(pairs) => {
            let ast = semantic::build_ast(pairs);
            let code = codegen::generate_code(ast);
            println!("Generated Code:\n{}", code);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
