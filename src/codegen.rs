use std::collections::HashMap;
use crate::parser::{ASTNode, Parser};
use crate::tokenizer::Token;
pub fn generate_code(tree: &ASTNode, symbol_map: &HashMap<String, Token>) -> String
{
    generate_main(tree, symbol_map)
}

pub fn generate_main(tree: &ASTNode, symbol_map: &HashMap<String, Token>) -> String
{
    println!("my tree: {:?}", tree);
    let mut code = String::new();
    code.push_str("#include <stdio.h> \n\n");
    code.push_str("int main(){ \n");
    code.push_str(generate_declaration(tree, symbol_map).leak());
    code.push_str("return 0; \n");
    code.push_str("\n }");
    code
}

pub fn generate_declaration(tree: &ASTNode, symbol_map: &HashMap<String, Token>) -> String
{
    match tree {
        ASTNode::Main { body } => {
            let mut c_code = String::new();
            for node in body {
                match node {
                    ASTNode::Number(value) => {
                        c_code.push_str(&value.to_string());
                    },
                    ASTNode::Identifier {
                        name,
                        data_type
                    } => {
                        c_code.push_str(name);
                    },
                    ASTNode::Declaration {
                        data_type,
                        identifier,
                        value,
                    } => {
                        let data_type_str = token_to_c_type(&data_type);
                        let identifier_str = token_to_c_identifier(&identifier);;
                        let value_str = to_c_code(*value.clone(), true);
                        c_code.push_str(&format!("{} {} = {};\n", data_type_str, identifier_str, value_str));
                    }
                    ASTNode::Print(token) => {
                        match token {
                            Token::StringLiteral(text) =>
                            {
                                c_code.push_str(&format!("printf(\"%s\", \"{}\"); \n", text))
                            },
                            Token::Identifier(ident) => {
                                let data_type = symbol_map.get(ident);
                                let data_type_str = token_to_c_print_type(data_type.unwrap());
                                c_code.push_str(&format!("printf(\"{}\", {}); \n", data_type_str, ident))
                            }
                            _ => { panic!("print parameter is invalid")}
                        }
                    }
                    ASTNode::BinaryOp { left, op, right } => {
                        let left_code = to_c_code(*left.clone(), true);
                        let operator = token_to_c_operator(op);
                        let right_code = to_c_code(*right.clone(), true);
                        c_code.push_str(&format!("({} {} {})", left_code, operator, right_code));
                    },
                    ASTNode::If {
                        block: Block
                    } => {
                        c_code.push_str(&to_c_code_if_stmt(node, symbol_map));
                    },
                    ASTNode::ElseIf {
                        block
                    } => {
                        c_code.push_str(&to_c_code_elseif_stmt(node, symbol_map));
                    }
                    ASTNode::Else{
                        block
                    } =>{
                        c_code.push_str(&to_c_code_else_stmt(node, symbol_map));
                    }
                    ASTNode::UniqueOp{
                        ident,
                        op
                    } =>{
                        let operator = if op == &Token::Increment { "++" } else { "--" };
                        c_code.push_str(&format!("{}{}; \n", ident, operator));
                    }
                    ASTNode::For {
                        declaration,
                        expression,
                        operation,
                        block
                    } => {
                        c_code.push_str(&for_stmt(node, symbol_map));
                    }
                    ASTNode::ArrayLiteral {
                        length,
                        values_data,
                        data_type,
                        ident } => {
                        let data_type_str = token_to_c_type(data_type);
                        let mut values_str = String::new();
                        for (i, value) in values_data.iter().enumerate() {
                            if i > 0 {
                                values_str.push_str(", ");
                            }
                            values_str.push_str(&to_c_code(value.clone(), false));
                        }

                        c_code.push_str(&format!("{} {}[{}] = {{ {} }}; \n", data_type_str, ident.clone(), length, values_str));
                    }
                    _ => panic!("Unsupported ASTNode in body of Main"),
                }
            }
            c_code
        }
        _ => panic!("Unsupported ASTNode type for generating C code"),
    }
}

pub fn to_c_code_if_stmt(tree: &ASTNode, symbol_map: &HashMap<String, Token>) -> String
{
    match tree {
        ASTNode::If { block } => {
            let mut c_code = String::new();
            c_code.push_str("if");
            for node in block {
                match node {
                    ASTNode::Number(value) => {
                        c_code.push_str(&value.to_string());
                    },
                    ASTNode::Identifier {
                        name,
                        data_type
                    } => {
                        c_code.push_str(name);
                    },
                    ASTNode::Declaration {
                        data_type,
                        identifier,
                        value,
                    } => {
                        let data_type_str = token_to_c_type(&data_type);
                        let identifier_str = token_to_c_identifier(&identifier);;
                        let value_str = to_c_code(*value.clone(), true);
                        c_code.push_str(&format!("{} {} = {};\n", data_type_str, identifier_str, value_str));
                    }
                    ASTNode::Print(token) => {
                        match token {
                            Token::StringLiteral(text) =>
                            {
                                c_code.push_str(&format!("printf(\"%s\", \"{}\"); \n", text))
                            },
                            Token::Identifier(ident) => {
                                let data_type = symbol_map.get(ident);
                                let data_type_str = token_to_c_print_type(data_type.unwrap());
                                c_code.push_str(&format!("printf(\"{}\", {}); \n", data_type_str, ident))
                            }
                            _ => { panic!("print parameter is invalid")}
                        }
                    }
                    ASTNode::BinaryOp { left, op, right } => {
                        let left_code = to_c_code(*left.clone(), true);
                        let operator = token_to_c_operator(&op);
                        let right_code = to_c_code(*right.clone(), true);
                        c_code.push_str(&format!("({} {} {})", left_code, operator, right_code));
                        c_code.push_str("\n { \n");
                    },
                    ASTNode::If {
                        block
                    } => {
                        c_code.push_str(&to_c_code_if_stmt(node, symbol_map));
                    }
                    ASTNode::UniqueOp{
                        ident,
                        op
                    } =>{
                        let operator = if op == &Token::Increment { "++" } else { "--" };
                        c_code.push_str(&format!("{}{}; \n", ident, operator));
                    }
                    ASTNode::ArrayLiteral {
                        length,
                        values_data,
                        data_type,
                        ident } => {
                        let data_type_str = token_to_c_type(data_type);
                        let mut values_str = String::new();
                        for (i, value) in values_data.iter().enumerate() {
                            if i > 0 {
                                values_str.push_str(", ");
                            }
                            values_str.push_str(&to_c_code(value.clone(), false));
                        }

                        c_code.push_str(&format!("{} {}[{}] = {{ {} }}; \n", data_type_str, ident.clone(), length, values_str));
                    }
                    _ => panic!("Unsupported ASTNode in body of Main"),
                }
            }
            c_code.push_str("}");
            c_code
        }
        _ => panic!("Unsupported ASTNode type for generating C code"),
    }
}

pub fn to_c_code_elseif_stmt(tree: &ASTNode, symbol_map: &HashMap<String, Token>) -> String
{
    match tree {
        ASTNode::ElseIf { block } => {
            let mut c_code = String::new();
            c_code.push_str("else if");
            for node in block {
                match node {
                    ASTNode::Number(value) => {
                        c_code.push_str(&value.to_string());
                    },
                    ASTNode::Identifier {
                        name,
                        data_type
                    } => {
                        c_code.push_str(name);
                    },
                    ASTNode::Declaration {
                        data_type,
                        identifier,
                        value,
                    } => {
                        let data_type_str = token_to_c_type(&data_type);
                        let identifier_str = token_to_c_identifier(&identifier);;
                        let value_str = to_c_code(*value.clone(), true);
                        c_code.push_str(&format!("{} {} = {};\n", data_type_str, identifier_str, value_str));
                        c_code.push_str(&format!("printf(\"%d\",{}); \n", identifier_str));
                    }
                    ASTNode::Print(token) => {
                        match token {
                            Token::StringLiteral(text) =>
                                {
                                    c_code.push_str(&format!("printf(\"%s\", \"{}\"); \n", text))
                                },
                            Token::Identifier(ident) => {
                                let data_type = symbol_map.get(ident);
                                let data_type_str = token_to_c_print_type(data_type.unwrap());
                                c_code.push_str(&format!("printf(\"{}\", {}); \n", data_type_str, ident))
                            }
                            _ => { panic!("print parameter is invalid")}
                        }
                    }
                    ASTNode::BinaryOp { left, op, right } => {
                        let left_code = to_c_code(*left.clone(), true);
                        let operator = token_to_c_operator(&op);
                        let right_code = to_c_code(*right.clone(), true);
                        c_code.push_str(&format!("({} {} {})", left_code, operator, right_code));
                        c_code.push_str("\n { \n");
                    },
                    ASTNode::Else {
                        block
                    } => {
                        c_code.push_str(&crate::codegen::to_c_code_else_stmt(node, symbol_map));
                    }
                    ASTNode::UniqueOp{
                        ident,
                        op
                    } =>{
                        let operator = if op == &Token::Increment { "++" } else { "--" };
                        c_code.push_str(&format!("{}{}; \n", ident, operator));
                    }
                    ASTNode::ArrayLiteral {
                        length,
                        values_data,
                        data_type,
                        ident } => {
                        let data_type_str = token_to_c_type(data_type);
                        let mut values_str = String::new();
                        for (i, value) in values_data.iter().enumerate() {
                            if i > 0 {
                                values_str.push_str(", ");
                            }
                            values_str.push_str(&to_c_code(value.clone(), false));
                        }

                        c_code.push_str(&format!("{} {}[{}] = {{ {} }}; \n", data_type_str, ident.clone(), length, values_str));
                    }
                    _ => panic!("Unsupported ASTNode in body of Main"),
                }
            }
            c_code.push_str("} \n");
            c_code
        }
        _ => panic!("Unsupported ASTNode type for generating C code"),
    }
}

pub fn to_c_code_else_stmt(tree: &ASTNode, symbol_map: &HashMap<String, Token>) -> String
{
    match tree {
        ASTNode::Else { block } => {
            let mut c_code = String::new();
            c_code.push_str("else");
            c_code.push_str("{ \n");
            for node in block {
                match node {
                    ASTNode::Number(value) => {
                        c_code.push_str(&value.to_string());
                    },
                    ASTNode::Identifier {
                        name,
                        data_type
                    } => {
                        c_code.push_str(name);
                    },
                    ASTNode::Declaration {
                        data_type,
                        identifier,
                        value,
                    } => {
                        let data_type_str = token_to_c_type(&data_type);
                        let identifier_str = token_to_c_identifier(&identifier);;
                        let value_str = to_c_code(*value.clone(), true);
                        c_code.push_str(&format!("{} {} = {};\n", data_type_str, identifier_str, value_str));
                    }
                    ASTNode::Print(token) => {
                        match token {
                            Token::StringLiteral(text) =>
                                {
                                    c_code.push_str(&format!("printf(\"%s\", \"{}\"); \n", text))
                                },
                            Token::Identifier(ident) => {
                                let data_type = symbol_map.get(ident);
                                let data_type_str = token_to_c_print_type(data_type.unwrap());
                                c_code.push_str(&format!("printf(\"{}\", {}); \n", data_type_str, ident))
                            }
                            _ => { panic!("print parameter is invalid")}
                        }
                    }
                    ASTNode::BinaryOp { left, op, right } => {
                        let left_code = to_c_code(*left.clone(), true);
                        let operator = token_to_c_operator(&op);
                        let right_code = to_c_code(*right.clone(), true);
                        c_code.push_str(&format!("({} {} {})", left_code, operator, right_code));
                    },
                    ASTNode::Else {
                        block
                    } => {
                        c_code.push_str(&crate::codegen::to_c_code_else_stmt(node, symbol_map));
                    }
                    ASTNode::UniqueOp{
                        ident,
                        op
                    } =>{
                        let operator = if op == &Token::Increment { "++" } else { "--" };
                        c_code.push_str(&format!("{}{}; \n", ident, operator));
                    }
                    ASTNode::ArrayLiteral {
                        length,
                        values_data,
                        data_type,
                        ident } => {
                        let data_type_str = token_to_c_type(data_type);
                        let mut values_str = String::new();
                        for (i, value) in values_data.iter().enumerate() {
                            if i > 0 {
                                values_str.push_str(", ");
                            }
                            values_str.push_str(&to_c_code(value.clone(), false));
                        }

                        c_code.push_str(&format!("{} {}[{}] = {{ {} }}; \n", data_type_str, ident.clone(), length, values_str));
                    }
                    _ => panic!("Unsupported ASTNode in body of Main"),
                }
            }
            c_code.push_str("} \n");
            c_code
        }
        _ => panic!("Unsupported ASTNode type for generating C code"),
    }
}

fn for_stmt(tree: &ASTNode, symbol_map: &HashMap<String, Token>) -> String
{
    match tree {
        ASTNode::For {
            declaration,
            expression,
            operation,
            block
        } => {
            let mut c_code = String::new();
            c_code.push_str("for(");
            c_code.push_str(&to_c_code(declaration.as_ref().clone(), true));
            c_code.push_str(";");
            c_code.push_str(&to_c_code(expression.as_ref().clone(), false));
            c_code.push_str(";");
            c_code.push_str(&to_c_code(operation.as_ref().clone(), true));
            c_code.push_str(") \n");
            c_code.push_str("{ \n");
            for node in block {
                match node {
                    ASTNode::Number(value) => {
                        c_code.push_str(&value.to_string());
                    },
                    ASTNode::Identifier {
                        name,
                        data_type
                    } => {
                        c_code.push_str(name);
                    },
                    ASTNode::Declaration {
                        data_type,
                        identifier,
                        value,
                    } => {
                        let data_type_str = token_to_c_type(&data_type);
                        let identifier_str = token_to_c_identifier(&identifier);;
                        let value_str = to_c_code(*value.clone(), false);
                        c_code.push_str(&format!("{} {} = {};\n", data_type_str, identifier_str, value_str));
                    }
                    ASTNode::Print(token) => {
                        match token {
                            Token::StringLiteral(text) =>
                                {
                                    c_code.push_str(&format!("printf(\"%s\", \"{}\"); \n", text))
                                },
                            Token::Identifier(ident) => {
                                let data_type = symbol_map.get(ident);
                                let data_type_str = token_to_c_print_type(data_type.unwrap());
                                c_code.push_str(&format!("printf(\"{}\", {}); \n", data_type_str, ident))
                            }
                            _ => { panic!("print parameter is invalid")}
                        }
                    }
                    ASTNode::BinaryOp { left, op, right } => {
                        let left_code = to_c_code(*left.clone(), false);
                        let operator = token_to_c_operator(&op);
                        let right_code = to_c_code(*right.clone(), false);
                        c_code.push_str(&format!("({} {} {})", left_code, operator, right_code));
                    },
                    ASTNode::If {
                        block
                    } => {
                        c_code.push_str(&to_c_code_if_stmt(node, symbol_map));
                    }
                    ASTNode::ElseIf {
                        block
                    } => {
                        c_code.push_str(&to_c_code_elseif_stmt(node, symbol_map));
                    }
                    ASTNode::Else {
                        block
                    } => {
                        c_code.push_str(&crate::codegen::to_c_code_else_stmt(node, symbol_map));
                    }
                    ASTNode::UniqueOp{
                        ident,
                        op
                    } =>{
                        let operator = if op == &Token::Increment { "++" } else { "--" };
                        c_code.push_str(&format!("{}{}; \n", ident, operator));
                    }
                    ASTNode::ArrayLiteral {
                        length,
                        values_data,
                        data_type,
                        ident } => {
                        let data_type_str = token_to_c_type(data_type);
                        let mut values_str = String::new();
                        for (i, value) in values_data.iter().enumerate() {
                            if i > 0 {
                                values_str.push_str(", ");
                            }
                            values_str.push_str(&to_c_code(value.clone(), false));
                        }

                        c_code.push_str(&format!("{} {}[{}] = {{ {} }}; \n", data_type_str, ident.clone(), length, values_str));
                    }
                    _ => panic!("Unsupported ASTNode in body of Main"),
                }
            }
            c_code.push_str("} \n");
            c_code
        }
        _ => panic!("Unsupported ASTNode type for generating C code"),
    }
}

pub fn token_to_c_operator(token: &Token) -> &'static str {
    match token {
        Token::Plus => "+",
        Token::Minus => "-",
        Token::Multiply => "*",
        Token::Divide => "/",
        Token::EqualThan => "==",
        Token::DifferentThan => "!=",
        Token::LessThan => "<",
        Token::LessEqualThan => "<=",
        Token::BiggerThan => ">",
        Token::BiggerThan => ">=",
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

pub fn token_to_c_print_type(token: &Token) -> &'static str {
    match token {
        Token::StringType => "%s",
        Token::IntegerType => "%d",
        Token::BooleanType => "%s",
        _ => panic!("Unsupported data type."),
    }
}
pub fn token_to_c_identifier(token: &Token) -> String {
    match token {
        Token::Identifier(name) => name.clone(),
        _ => panic!("Expected Identifier token."),
    }
}
pub fn to_c_code(ast: ASTNode, without_paren: bool) -> String {
    println!("ast to_c_code: {:?}", ast);
    match ast {
        ASTNode::Number(value) => value.to_string(),
        ASTNode::Identifier {name, data_type}=> name.clone(),
        ASTNode::StringLiteral(value) => format!("\"{}\"", value),
        ASTNode::BinaryOp { left, op, right } => {
            let left_code = to_c_code(*left, without_paren);
            let right_code = to_c_code(*right, without_paren);
            let operator = token_to_c_operator(&op);
            if without_paren == true {
                format!("({} {} {})", left_code, operator, right_code)
            }else{
                format!("{} {} {}", left_code, operator, right_code)
            }
        },
        ASTNode::Declaration { data_type, identifier, value } => {
            let data_type_str = token_to_c_type(&data_type);
            let value_code = to_c_code(*value, without_paren);
            let ident_str = if let Token::Identifier(ident) = identifier
            { ident.clone() }
            else{ panic!("Invalid identifier: {:?}", identifier)};
            format!("{} {} = {}", data_type_str, ident_str, value_code)
        },
        ASTNode::UniqueOp{
            ident,
            op
        } =>{
            let operator = if op == Token::Increment { "++" } else { "--" };
            format!("{}{}", ident, operator)
        }
        _ => panic!("Unexpected AST node type"),
    }
}