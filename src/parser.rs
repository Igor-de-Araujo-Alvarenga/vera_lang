use crate::tokenizer::Token;

pub struct Parser{
    tokens: Vec<Token>,
    current: usize,
}
#[derive(Debug, Clone)]
pub enum ASTNode {
    Identifier(String),
    Declaration {
        data_type: Token,
        identifier: Token,
        value: Box<ASTNode>,
    },
    Number(i32),
    LogicOperator(String),
    BinaryOp {
        left: Box<ASTNode>,
        op: Token,
        right: Box<ASTNode>
    },
    Main {
        body: Vec<ASTNode>
    },
    Print(String)
}

impl Parser
{
    pub fn new(tokens: Vec<Token>) -> Self
    {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> ASTNode
    {
        if self.match_token(&[Token::Main]) {
            self.parse_main()
        } else {
            self.parse_expression().unwrap()
        }
    }
    fn parse_main(&mut self) -> ASTNode
    {
        self.consume(&Token::LParen);
        self.consume(&Token::RParen);
        self.consume(&Token::LineBreak);
        self.consume(&Token::LBrace);
        self.consume(&Token::LineBreak);
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
                },//TODO: parse expressions
                _ => { panic!("Unsupported feature.")}
            }
            self.consume(&Token::LineBreak);
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
            self.consume(&Token::LParen);
            logic_expr.unwrap()
        }
        else if let Token::Identifier(ident) = self.advance()
        {
            ASTNode::LogicOperator(ident.clone())
        }
        else {
            panic!("Expected logic expression")
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
        let data_type = self.tokens[self.current].clone();
        self.advance();
        let identifier = self.peek().clone();
        println!("identificador: {:?}", identifier);
        self.advance();
        self.consume(&Token::Assignment);
        let value = self.parse_expression();
        Some(ASTNode::Declaration {
            data_type,
            identifier: identifier,
            value: Box::new(value.unwrap()),
        })
    }

    fn parse_print(&mut self) -> Option<ASTNode>
    {
        self.consume(&Token::Print);
        self.consume(&Token::LParen);
        if let Token::StringLiteral(text) = self.advance()
        {
            let print_stmt = ASTNode::Print(text.clone());
            self.consume(&Token::RParen);
            Some(print_stmt)
        }else{
            panic!("Expected string literal");
        }
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