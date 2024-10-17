// tests/scanner_test.rs
use formula::scanner::Scanner;
use formula::token::TokenType;

fn scan_tokens(source: &str) -> Vec<TokenType> {
    let mut scanner = Scanner::new(source.to_string());
    scanner.scan().into_iter().map(|token| token.token_type).collect()
}

#[test]
fn test_scan_empty() {
    let tokens = scan_tokens("");
    assert_eq!(tokens, vec![TokenType::Eof]);
}

#[test]
fn test_scan_single_digit() {
    let tokens = scan_tokens("5");
    assert_eq!(tokens, vec![TokenType::Integer, TokenType::Eof]);
}

#[test]
fn test_scan_single_alpha() {
    let tokens = scan_tokens("a");
    assert_eq!(tokens, vec![TokenType::Identifier, TokenType::Eof]);
}

#[test]
fn test_scan_mixed_cell() {
    let tokens = scan_tokens("A$1");
    assert_eq!(tokens, vec![TokenType::MixedCell, TokenType::Eof]);
}

#[test]
fn test_scan_absolute_cell() {
    let tokens = scan_tokens("$A$1");
    assert_eq!(tokens, vec![TokenType::AbsoluteCell, TokenType::Eof]);
}

#[test]
fn test_scan_r1c1() {
    let tokens = scan_tokens("R1C1");
    assert_eq!(tokens, vec![TokenType::R1C1, TokenType::Eof]);
}

#[test]
fn test_scan_string() {
    let tokens = scan_tokens("\"hello\"");
    assert_eq!(tokens, vec![TokenType::String, TokenType::Eof]);
}

#[test]
fn test_scan_float() {
    let tokens = scan_tokens("3.14");
    assert_eq!(tokens, vec![TokenType::Float, TokenType::Eof]);
}

#[test]
fn test_scan_scientific_notation() {
    let tokens = scan_tokens("1.23e10");
    assert_eq!(tokens, vec![TokenType::Float, TokenType::Eof]);
}

#[test]
fn test_scan_operators() {
    let tokens = scan_tokens("+-*/^&%");
    assert_eq!(
        tokens,
        vec![
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Star,
            TokenType::Slash,
            TokenType::Exponent,
            TokenType::Concatenate,
            TokenType::Percent,
            TokenType::Eof
        ]
    );
}