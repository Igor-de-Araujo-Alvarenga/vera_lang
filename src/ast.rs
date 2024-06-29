#[derive(Debug, PartialEq)]
pub enum Statement {
    Assignment(String, Expression),
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(i64),
    Identifier(String),
    BinaryOp(Box<Expression>, Operator, Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}
