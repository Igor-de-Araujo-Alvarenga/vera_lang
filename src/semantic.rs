use crate::ast::{Expression, Operator, Statement};
use pest::iterators::Pairs;
use crate::parser::Rule;

pub fn build_ast(pairs: Pairs<Rule>) -> Vec<Statement> {
    let mut statements = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::statement => {
                statements.push(build_statement(pair.into_inner()));
            }
            _ => unreachable!(),
        }
    }
    statements
}

fn build_statement(pairs: Pairs<Rule>) -> Statement {
    let pair = pairs.peek().unwrap();
    match pair.as_rule() {
        Rule::assignment => {
            let mut inner_pairs = pair.into_inner();
            let identifier = inner_pairs.next().unwrap().as_str().to_string();
            let expression = build_expression(inner_pairs.next().unwrap().into_inner());
            Statement::Assignment(identifier, expression)
        }
        Rule::expression => {
            let expression = build_expression(pair.into_inner());
            Statement::Expression(expression)
        }
        _ => unreachable!(),
    }
}

fn build_expression(pairs: Pairs<Rule>) -> Expression {
    let mut iter = pairs.peekable();
    let mut left = build_term(iter.next().unwrap().into_inner());

    while let Some(pair) = iter.next() {
        let operator = match pair.as_rule() {
            Rule::add => Operator::Add,
            Rule::subtract => Operator::Subtract,
            _ => unreachable!(),
        };
        let right = build_term(iter.next().unwrap().into_inner());
        left = Expression::BinaryOp(Box::new(left), operator, Box::new(right));
    }
    left
}

fn build_term(pairs: Pairs<Rule>) -> Expression {
    let mut iter = pairs.peekable();
    let mut left = build_factor(iter.next().unwrap().into_inner());

    while let Some(pair) = iter.next() {
        let operator = match pair.as_rule() {
            Rule::multiply => Operator::Multiply,
            Rule::divide => Operator::Divide,
            _ => unreachable!(),
        };
        let right = build_factor(iter.next().unwrap().into_inner());
        left = Expression::BinaryOp(Box::new(left), operator, Box::new(right));
    }
    left
}

fn build_factor(pairs: Pairs<Rule>) -> Expression {
    let pair = pairs.peek().unwrap();
    match pair.as_rule() {
        Rule::number => Expression::Number(pair.as_str().parse().unwrap()),
        Rule::identifier => Expression::Identifier(pair.as_str().to_string()),
        Rule::expression => build_expression(pair.into_inner()),
        _ => unreachable!(),
    }
}
