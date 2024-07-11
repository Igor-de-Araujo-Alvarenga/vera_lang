use crate::parser::ASTNode;
use crate::tokenizer::Token;
pub fn generate_code(tree: &ASTNode) -> String
{
    generate_main(tree)
}

pub fn generate_main(tree: &ASTNode) -> String
{
    println!("my tree: {:?}", tree);
    let mut code = String::new();
    code.push_str("#include <stdio.h> \n\n");
    code.push_str("int main(){ \n");
    code.push_str(generate_expression(tree).leak());
    code.push_str("return 0; \n");
    code.push_str("\n }");
    code
}

pub fn generate_expression(tree: &ASTNode) -> String
{
    match tree {
        ASTNode::Main { body } => {
            let mut c_code = String::new();
            for node in body {
                match node {
                    ASTNode::Declaration {
                        data_type,
                        identifier,
                        value,
                    } => {
                        let data_type_str = token_to_c_type(&data_type)/* Convert data_type Token to string */;
                        let identifier_str = token_to_c_identifier(&identifier);/* Convert identifier Token to string */;
                        let value_str = match **value {
                            ASTNode::Number(n) => n.to_string(),
                            _ => panic!("Unsupported value type in declaration"),
                        };
                        c_code.push_str(&format!("{} {} = {};\n", data_type_str, identifier_str, value_str));
                        c_code.push_str(&format!("printf(\"%d\",{}); \n", identifier_str));
                        c_code.push_str(&format!("int a; \n scanf(\"%d\", a); \n"));
                    }
                    // Handle other ASTNode variants as needed
                    _ => panic!("Unsupported ASTNode in body of Main"),
                }
            }
            c_code
        }
        _ => panic!("Unsupported ASTNode type for generating C code"),
    }
}

pub fn token_to_c_type(token: &Token) -> &'static str {
    match token {
        Token::StringType => "char*",
        Token::IntegerType => "int",
        Token::BooleanType => "bool",
        _ => panic!("Unsupported data type"),
    }
}

// Helper function to convert Token to C identifier string
pub fn token_to_c_identifier(token: &Token) -> String {
    match token {
        Token::Identifier(name) => name.clone(),
        _ => panic!("Expected Identifier token"),
    }
}

