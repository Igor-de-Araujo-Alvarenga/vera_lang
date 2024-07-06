use crate::tokenizer::Token;

pub struct Parser{
    tokens: Vec<Token>,
    current: usize,
}
#[derive(Debug, Clone)]
pub enum ASTNode {
    Number(i32),
    BinaryOp {
        left: Box<ASTNode>,
        op: Token,
        right: Box<ASTNode>
    },
    Main {
        body: Box<ASTNode>
    }
}

impl Parser
{
    pub fn new(tokens: Vec<Token>) -> Self
    {
        Parser{ tokens, current: 0}
    }

    pub fn parse(&mut self) -> ASTNode
    {
        if self.match_token(&[Token::Main]) {
            self.parse_main()
        } else {
            self.parse_expression()
        }
    }
    fn parse_main(&mut self) -> ASTNode
    {
        self.consume(&Token::LParen);
        self.consume(&Token::RParen);
        self.consume(&Token::LBrace);
        let body = self.parse_expression();
        self.consume(&Token::RBrace);
        ASTNode::Main {
            body: Box::new(body)
        }
    }
    fn parse_expression(&mut self) -> ASTNode
    {
        let mut node = self.parse_term();
        while self.match_token(&[Token::Plus, Token::Minus])
        {
            let op = self.previous().clone();
            let right = self.parse_term();
            node = ASTNode::BinaryOp{
                left: Box::new(node),
                op,
                right: Box::new(right),
            };
        }
        node
    }
    fn parse_term(&mut self) -> ASTNode
    {
        let mut node = self.parse_factor();
        while self.match_token(&[Token::Multiply, Token::Divide])
        {
            let op = self.previous().clone();
            let right = self.parse_factor();
            node = ASTNode::BinaryOp {
                left : Box::new(node),
                op,
                right: Box::new(right)
            };
        }
        node
    }

    fn parse_factor(&mut self) -> ASTNode
    {
        if self.match_token(&[Token::LParen]){
            let expr = self.parse_expression();
            self.consume(&Token::RParen);
            expr
        } else if let Token::Number(num) = self.advance() {
            ASTNode::Number(num.parse().unwrap())
        } else {
            panic!("Expected expression")
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
            panic!("Expected {:?}", token_type);
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