use std::fmt::format;
use crate::tokenizer::Token;
use crate::tokenizer::Token::Identifier;
use std::collections::HashMap;
pub struct Parser{
    tokens: Vec<Token>,
    current: usize,
    pub symbol_table: HashMap<String, Token>,
}
#[derive(Debug, Clone)]
pub enum ASTNode {
    Identifier{
        name: String,
        data_type: Token
    },
    Declaration {
        data_type: Token,
        identifier: Token,
        value: Box<ASTNode>,
    },
    Number(i32),
    StringLiteral(String),
    BinaryOp {
        left: Box<ASTNode>,
        op: Token,
        right: Box<ASTNode>
    },
    Main {
        body: Vec<ASTNode>
    },
    Print(Token),
    If {
        block: Vec<ASTNode>
    },
    ElseIf{
        block: Vec<ASTNode>
    },
    Else{
        block: Vec<ASTNode>
    }
}

impl Parser
{
    pub fn new(tokens: Vec<Token>) -> Self
    {
        Parser { tokens, current: 0, symbol_table: HashMap::new() }
    }

    fn parse_and_push<F>(&mut self, body: &mut Vec<ASTNode>, token: Token, parse_fn: F)
    where
        F: Fn(&mut Self) -> Option<ASTNode>,
    {
        if self.peek() == &token {
            if let Some(node) = parse_fn(self) {
                body.push(node);
            }
        }
    }
    pub fn parse(&mut self) -> ASTNode {
        self.consume(&Token::Main);
        self.consume(&Token::LParen);
        self.consume(&Token::RParen);
        self.consume(&Token::LBrace);
        let mut body = Vec::new();
        while !self.check(&Token::RBrace) {
            self.parse_and_push(&mut body, Token::Print, Self::parse_print);
            self.parse_and_push(&mut body, Token::If, Self::parse_if_condition);
            self.parse_and_push(&mut body, Token::ElseIf, Self::parse_else_if_condition);
            self.parse_and_push(&mut body, Token::Else, Self::parse_else_condition);
            self.parse_and_push(&mut body, Token::StringType, Self::parse_declaration);
            self.parse_and_push(&mut body, Token::IntegerType, Self::parse_declaration);
            self.parse_and_push(&mut body, Token::BooleanType, Self::parse_declaration);
        }
        self.consume(&Token::RBrace);
        ASTNode::Main { body }
    }

    fn parse_if_condition(&mut self) -> Option<ASTNode> {
        let mut if_ast = Vec::new();
        self.consume(&Token::If);
        let logic_expression = self.parse_logic_expression();
        if_ast.push(logic_expression.unwrap());
        self.consume(&Token::LBrace);
        while !self.check(&Token::RBrace) {
            self.parse_and_push(&mut if_ast, Token::Print, Self::parse_print);
            self.parse_and_push(&mut if_ast, Token::If, Self::parse_if_condition);
            self.parse_and_push(&mut if_ast, Token::ElseIf, Self::parse_else_if_condition);
            self.parse_and_push(&mut if_ast, Token::Else, Self::parse_else_condition);
            self.parse_and_push(&mut if_ast, Token::StringType, Self::parse_declaration);
            self.parse_and_push(&mut if_ast, Token::IntegerType, Self::parse_declaration);
            self.parse_and_push(&mut if_ast, Token::BooleanType, Self::parse_declaration);
        }
        self.consume(&Token::RBrace);
        Some(ASTNode::If { block: if_ast })
    }

    fn parse_else_if_condition(&mut self) -> Option<ASTNode> {
        let mut ast = Vec::new();
        self.consume(&Token::ElseIf);
        let logic_expression = self.parse_logic_expression();
        ast.push(logic_expression.unwrap());
        self.consume(&Token::LBrace);
        while !self.check(&Token::RBrace) {
            self.parse_and_push(&mut ast, Token::Print, Self::parse_print);
            self.parse_and_push(&mut ast, Token::If, Self::parse_if_condition);
            self.parse_and_push(&mut ast, Token::ElseIf, Self::parse_else_if_condition);
            self.parse_and_push(&mut ast, Token::Else, Self::parse_else_condition);
            self.parse_and_push(&mut ast, Token::StringType, Self::parse_declaration);
            self.parse_and_push(&mut ast, Token::IntegerType, Self::parse_declaration);
            self.parse_and_push(&mut ast, Token::BooleanType, Self::parse_declaration);
        }
        self.consume(&Token::RBrace);
        Some(ASTNode::ElseIf { block: ast })
    }

    fn parse_else_condition(&mut self) -> Option<ASTNode> {
        let mut else_ast = Vec::new();
        self.consume(&Token::Else);
        self.consume(&Token::LBrace);
        while !self.check(&Token::RBrace) {
            self.parse_and_push(&mut else_ast, Token::Print, Self::parse_print);
            self.parse_and_push(&mut else_ast, Token::If, Self::parse_if_condition);
            self.parse_and_push(&mut else_ast, Token::ElseIf, Self::parse_else_if_condition);
            self.parse_and_push(&mut else_ast, Token::Else, Self::parse_else_condition);
            self.parse_and_push(&mut else_ast, Token::StringType, Self::parse_declaration);
            self.parse_and_push(&mut else_ast, Token::IntegerType, Self::parse_declaration);
            self.parse_and_push(&mut else_ast, Token::BooleanType, Self::parse_declaration);
        }
        self.consume(&Token::RBrace);
        Some(ASTNode::Else { block: else_ast })
    }

    fn parse_main2(&mut self) -> ASTNode
    {
        self.consume(&Token::LParen);
        self.consume(&Token::RParen);
        self.consume(&Token::LBrace);
        let mut body = Vec::new();
        while !self.check(&Token::RBrace) {
            match self.peek() {
                Token::Print => {
                    if let Some(print_stmt) = self.parse_print() {
                        body.push(print_stmt);
                    }
                },
                Token::IntegerType | Token::StringType | Token::BooleanType => {
                    if let Some(declaration) = self.parse_declaration() {
                        body.push(declaration);
                    }
                },
                Token::If => {
                    if let Some(if_stmt) = self.parse_if_condition(){
                        body.push(if_stmt);
                    }
                }
                Token::ElseIf => {
                    if let Some(else_if_stmt) = self.parse_else_if_condition(){
                        body.push(else_if_stmt);
                    }
                }
                Token::Else => {
                    if let Some(else_stmt ) = self.parse_else_condition(){
                        body.push(else_stmt);
                    }
                }
                _ => { panic!("Unsupported feature: {:?}.", self.peek())}
            }
        }
        self.consume(&Token::RBrace);
        ASTNode::Main {
            body: body,
        }
    }
    fn parse_expression(&mut self) -> Option<ASTNode>
    {
        let mut node = self.parse_term();
        while self.match_token(&[Token::Plus, Token::Minus])
        {
            let op = self.previous().clone();
            let right = self.parse_term();
            node = ASTNode::BinaryOp {
                left: Box::new(node),
                op,
                right: Box::new(right),
            };
        }
        Some(node)
    }
    fn parse_logic_expression(&mut self) -> Option<ASTNode>
    {
        let mut node = self.parse_logic_factor();
        while self.match_token(&[Token::LessThan, Token::LessEqualThan, Token::BiggerEqualThan,
        Token::BiggerThan, Token::DifferentThan, Token::EqualThan])
        {
            let op = self.previous().clone();
            let right = self.parse_logic_factor();
            node = ASTNode::BinaryOp {
                left : Box::new(node),
                op,
                right: Box::new(right)
            }
        }
        Some(node)
    }
    fn parse_logic_factor(&mut self) -> ASTNode
    {
        if self.match_token(&[Token::LParen])
        {
            let logic_expr = self.parse_logic_expression();
            self.consume(&Token::RParen);
            logic_expr.unwrap()
        }
        else if let Token::Identifier(ident) = self.peek()
        {
            let ident_clone = ident.clone();
            let data_type = self.symbol_table.get(&ident_clone).unwrap();
            let ident = ASTNode::Identifier{
                name: ident_clone.clone(),
                data_type: data_type.clone()
            };
            self.consume(&Token::Identifier(ident_clone.to_string()));
            ident
        }
        else if let Token::Number(num) = self.peek()
        {
            let num_string = num.to_string();
            self.consume(&Token::Number(num_string.parse().unwrap()));
            ASTNode::Number(num_string.parse().unwrap())
        }
        else {
            panic!("Expected logic expression {:?}", self.tokens[self.current])
        }
    }
    fn parse_term(&mut self) -> ASTNode
    {
        let mut node = self.parse_factor();
        while self.match_token(&[Token::Multiply, Token::Divide])
        {
            let op = self.previous().clone();
            let right = self.parse_factor();
            node = ASTNode::BinaryOp {
                left: Box::new(node),
                op,
                right: Box::new(right)
            };
        }
        node
    }

    fn parse_factor(&mut self) -> ASTNode
    {
        if self.match_token(&[Token::LParen]) {
            let expr = self.parse_expression();
            self.consume(&Token::RParen);
            expr.unwrap()
        } else if let Token::Number(num) = self.advance() {
            ASTNode::Number(num.parse().unwrap())
        } else {
            panic!("Expected expression")
        }
    }

    fn parse_declaration(&mut self) -> Option<ASTNode> {
        let data_type = self.advance().clone();
        let identifier_token = self.advance();
        let identifier = match identifier_token {
            Token::Identifier(id) => id.clone(),
            _ => panic!("Expected identifier, found {:?}", identifier_token),
        };
        self.consume(&Token::Assignment);
        let value = match self.peek() {
            Token::LParen => {
                let value = self.parse_expression();
                match value {
                    Some(val) => val,
                    None => panic!("Expected expression inside parentheses."),
                }
            }
            Token::StringLiteral(str) => {
                let str_literal = str.clone();
                self.advance();
                ASTNode::StringLiteral(str_literal)
            }
            Token::Number(num) => {
                let ast = ASTNode::Number(num.parse().unwrap());
                self.advance();
                ast
            }
            _ => panic!("Expected value for declaration."),
        };
        self.symbol_table.insert(identifier.clone(), data_type.clone());
        Some(ASTNode::Declaration {
            data_type,
            identifier: Token::Identifier(identifier),
            value: Box::new(value),
        })
    }

    fn parse_print(&mut self) -> Option<ASTNode>
    {
        self.consume(&Token::Print);
        self.consume(&Token::LParen);
        let print_stmt = match self.peek() {
            Token::StringLiteral(text) =>
            {
                let print_stmt = ASTNode::Print(Token::StringLiteral(text.clone()));
                print_stmt
            }
            Token::Number(num) =>
            {
                let print_stmt = ASTNode::Print(Token::Number(num.parse().unwrap()));
                print_stmt
            }
            Token::Identifier(text) =>
            {
                let print_stmt = ASTNode::Print(Identifier(text.clone()));
                print_stmt
            }
            _ => { panic!("Expected string literal"); }
        };
        self.advance();
        self.consume(&Token::RParen);
        Some(print_stmt)
    }

    fn parse_if_condition1(&mut self) -> Option<ASTNode>
    {
        let mut if_ast = Vec::new();
        self.consume(&Token::If);
        let logic_expression = self.parse_logic_expression();
        if_ast.push(logic_expression.unwrap());
        self.consume(&Token::LBrace);
        while !self.check(&Token::RBrace) {
            match self.peek() {
                Token::Print => {
                    if let Some(print_stmt) = self.parse_print() {
                        if_ast.push(print_stmt);
                    }
                },
                Token::IntegerType | Token::StringType | Token::BooleanType => {
                    if let Some(declaration) = self.parse_declaration() {
                        if_ast.push(declaration);
                    }
                },
                Token::If => {
                    if let Some(if_stmt) = self.parse_if_condition(){
                        if_ast.push(if_stmt);
                    }
                }
                Token::ElseIf => {
                    if let Some(else_if_stmt) = self.parse_else_if_condition(){
                        if_ast.push(else_if_stmt);
                    }
                }
                Token::Else => {
                    if let Some(else_stmt ) = self.parse_else_condition(){
                        if_ast.push(else_stmt);
                    }
                }
                _ => {
                    panic!("Unsupported feature.")
                }
            }
        }
        self.consume(&Token::RBrace);
        Some(ASTNode::If{
            block: if_ast
        })
    }

    fn parse_else_if_condition2(&mut self) -> Option<ASTNode>
    {
        let mut ast = Vec::new();
        self.consume(&Token::ElseIf);
        let logic_expression = self.parse_logic_expression();
        ast.push(logic_expression.unwrap());
        self.consume(&Token::LBrace);
        while !self.check(&Token::RBrace) {
            match self.peek() {
                Token::Print => {
                    if let Some(print_stmt) = self.parse_print() {
                        ast.push(print_stmt);
                    }
                },
                Token::IntegerType | Token::StringType | Token::BooleanType => {
                    if let Some(declaration) = self.parse_declaration() {
                        ast.push(declaration);
                    }
                },
                Token::If => {
                    if let Some(if_stmt) = self.parse_if_condition(){
                        ast.push(if_stmt);
                    }
                }
                Token::ElseIf => {
                    if let Some(else_if_stmt) = self.parse_else_if_condition(){
                        ast.push(else_if_stmt);
                    }
                }
                Token::Else => {
                    if let Some(else_stmt ) = self.parse_else_condition(){
                        ast.push(else_stmt);
                    }
                }
                _ => {
                    panic!("Unsupported feature.")
                }
            }
        }
        self.consume(&Token::RBrace);
        Some(ASTNode::ElseIf{
            block: ast
        })
    }

    fn parse_else_condition2(&mut self) -> Option<ASTNode>
    {
        let mut else_ast = Vec::new();
        self.consume(&Token::Else);
        self.consume(&Token::LBrace);
        while !self.check(&Token::RBrace) {
            match self.peek() {
                Token::Print => {
                    if let Some(print_stmt) = self.parse_print() {
                        else_ast.push(print_stmt);
                    }
                },
                Token::IntegerType | Token::StringType | Token::BooleanType => {
                    if let Some(declaration) = self.parse_declaration() {
                        else_ast.push(declaration);
                    }
                },
                Token::If => {
                    if let Some(if_stmt) = self.parse_if_condition(){
                        else_ast.push(if_stmt);
                    }
                }
                Token::ElseIf => {
                    if let Some(else_if_stmt) = self.parse_else_if_condition(){
                        else_ast.push(else_if_stmt);
                    }
                }
                Token::Else => {
                    if let Some(else_stmt ) = self.parse_else_condition(){
                        else_ast.push(else_stmt);
                    }
                }
                _ => {
                    panic!("Unsupported feature.")
                }
            }
        }
        self.consume(&Token::RBrace);
        Some(ASTNode::Else{
            block: else_ast
        })
    }
    fn match_token(&mut self, types: &[Token]) -> bool
    {
        for token_type in types {
            if self.check(&token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_types: &Token) -> bool
    {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(self.peek()) == std::mem::discriminant(token_types)
        }
    }
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    fn consume(&mut self, token_type: &Token) {
        if self.check(token_type) {
            self.advance();
        } else {
            panic!("Expected {:?} but is found {:?}", token_type, self.peek());
        }
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}