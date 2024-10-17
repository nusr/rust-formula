use crate::expression::LiteralExpression;
use crate::token::Token;
use crate::expression::Expression;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Box<dyn Expression>> {
        let mut result: Vec<Box<dyn Expression>> = Vec::new();
        result.push(Box::new(self.expression()));
        result
    }

    fn expression(&mut self) -> LiteralExpression {
        let token = self.tokens[self.current].clone();
        self.current += 1;
        LiteralExpression::new(token)
    }
}
