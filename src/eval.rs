use crate::parser::Parser;
use crate::scanner::Scanner;

pub fn parse_formula(formula: String) {
    let mut scanner = Scanner::new(formula);
    let tokens = scanner.scan();
    let mut parser = Parser::new(tokens);
    parser.parse();
}
