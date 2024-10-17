use crate::expression::{BinaryExpression, Expression, LiteralExpression, Visitor};

pub struct Interpreter;

impl Visitor for Interpreter {
    fn visit_binary_expression(&mut self, expr: &BinaryExpression) {
        println!("{}", expr.to_string());
    }

    fn visit_literal_expression(&mut self, _expr: &LiteralExpression) {
        println!("Visiting unary expression");
    }
}
