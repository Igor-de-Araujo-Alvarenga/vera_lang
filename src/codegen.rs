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
    code.push_str(generate_declaration(tree).leak());
    code.push_str(&format!("int a; \n scanf(\"%d\", a); \n"));
    code.push_str("return 0; \n");
    code.push_str("\n }");
    code
}

pub fn generate_declaration(tree: &ASTNode) -> String
{
    match tree {
        ASTNode::Main { body } => {
            let mut c_code = String::new();
            for node in body {
                match node {
                    ASTNode::Number(value) => {
                        c_code.push_str(&value.to_string());
                    },
                    ASTNode::Identifier(name) => {
                        c_code.push_str(name);
                    },
                    ASTNode::Declaration {
                        data_type,
                        identifier,
                        value,
                    } => {
                        let data_type_str = token_to_c_type(&data_type);
                        let identifier_str = token_to_c_identifier(&identifier);;
                        let value_str = to_c_code(*value.clone());
                        c_code.push_str(&format!("{} {} = {};\n", data_type_str, identifier_str, value_str));
                        c_code.push_str(&format!("printf(\"%d\",{}); \n", identifier_str));
                    }
                    ASTNode::Print(text) => {
                        c_code.push_str(&format!("printf(\"%s\", \"{}\"); \n", text))
                    }
                    ASTNode::BinaryOp { left, op, right } => {
                        let left_code = to_c_code(*left.clone());
                        let operator = token_to_c_math_operator(op);
                        let right_code = to_c_code(*right.clone());
                        c_code.push_str(&format!("({} {} {})", left_code, operator, right_code));
                    },
                    _ => panic!("Unsupported ASTNode in body of Main"),
                }
            }
            c_code
        }
        _ => panic!("Unsupported ASTNode type for generating C code"),
    }
}
pub fn token_to_c_math_operator(token: &Token) -> &'static str {
    match token {
        Token::Plus => "+",
        Token::Minus => "-",
        Token::Multiply => "*",
        Token::Divide => "/",
        _ => panic!("Unsupported math operator.")
    }
}
pub fn token_to_c_type(token: &Token) -> &'static str {
    match token {
        Token::StringType => "char*",
        Token::IntegerType => "int",
        Token::BooleanType => "bool",
        _ => panic!("Unsupported data type."),
    }
}

pub fn token_to_c_identifier(token: &Token) -> String {
    match token {
        Token::Identifier(name) => name.clone(),
        _ => panic!("Expected Identifier token."),
    }
}
pub fn to_c_code(ast: ASTNode) -> String {
    match ast {
        ASTNode::Number(value) => value.to_string(),
        ASTNode::Identifier(name) => name.clone(),
        ASTNode::BinaryOp { left, op, right } => {
            let left_code = to_c_code(*left);
            let right_code = to_c_code(*right);
            let operator = token_to_c_math_operator(&op);
            format!("({} {} {})", left_code, operator, right_code)
        },
        ASTNode::Declaration { data_type, identifier, value } => {
            let data_type_str = token_to_c_type(&data_type);
            let value_code = to_c_code(*value);
            format!("{} {:?} = {};", data_type_str, identifier, value_code)
        },
        _ => panic!("Unexpected AST node type"),
    }
}