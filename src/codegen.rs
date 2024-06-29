use crate::ast::{Expression, Operator, Statement};

pub fn generate_code(statements: Vec<Statement>) -> String {
    let mut code = String::new();
    for statement in statements {
        code.push_str(&generate_statement(&statement));
        code.push_str(";\n");
    }
    code
}

fn generate_statement(statement: &Statement) -> String {
    match statement {
        Statement::Assignment(identifier, expression) => {
            format!("{} = {}", identifier, generate_expression(expression))
        }
        Statement::Expression(expression) => generate_expression(expression),
    }
}

fn generate_expression(expression: &Expression) -> String {
    match expression {
        Expression::Number(value) => value.to_string(),
        Expression::Identifier(identifier) => identifier.to_string(),
        Expression::BinaryOp(left, operator, right) => {
            format!(
                "{} {} {}",
                generate_expression(left),
                generate_operator(operator),
                generate_expression(right)
            )
        }
    }
}

fn generate_operator(operator: &Operator) -> &str {
    match operator {
        Operator::Add => "+",
        Operator::Subtract => "-",
        Operator::Multiply => "*",
        Operator::Divide => "/",
    }
}
