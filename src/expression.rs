use crate::token::Token;

pub trait Visitor {
    fn visit_binary_expression(&mut self, expr: &BinaryExpression);
    // fn visit_unary_expression(&mut self, expr: &UnaryExpression);
    // fn visit_post_unary_expression(&mut self, expr: &PostUnaryExpression);
    fn visit_literal_expression(&mut self, expr: &LiteralExpression);
    // fn visit_cell_expression(&mut self, expr: &CellExpression);
    // fn visit_r1c1_expression(&mut self, expr: &R1C1Expression);
    // fn visit_cell_range_expression(&mut self, expr: &CellRangeExpression);
    // fn visit_call_expression(&mut self, expr: &CallExpression);
    // fn visit_group_expression(&mut self, expr: &GroupExpression);
    // fn visit_token_expression(&mut self, expr: &TokenExpression);
    // fn visit_array_expression(&mut self, expr: &ArrayExpression);
}

pub trait Expression {
    fn accept(&self, visitor: &mut dyn Visitor);
    fn to_string(&self) -> String;
}

pub struct BinaryExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: Token,
}

impl BinaryExpression {
    pub fn new(left: Box<dyn Expression>, operator: Token, right: Box<dyn Expression>) -> Self {
        BinaryExpression {
            left,
            operator,
            right,
        }
    }
}

impl Expression for BinaryExpression {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_binary_expression(self);
    }

    fn to_string(&self) -> String {
        self.left.to_string().to_string()
    }
}

pub struct LiteralExpression {
    pub value: Token,
}

impl LiteralExpression {
    pub fn new(value: Token) -> Self {
        LiteralExpression { value }
    }
}

impl Expression for LiteralExpression {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_literal_expression(self);
    }

    fn to_string(&self) -> String {
        self.value.to_string().to_string()
    }
}
