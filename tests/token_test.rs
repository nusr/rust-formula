// tests/token_test.rs
use formula::token::Token;
use formula::token::TokenType;

#[test]
fn test_token_creation() {
    let token = Token::new(TokenType::Plus, "+".to_string());
    assert_eq!(token.token_type, TokenType::Plus);
    assert_eq!(token.to_string(), "+");
}

#[test]
fn test_token_equality() {
    let token1 = Token::new(TokenType::Minus, "-".to_string());
    let token2 = Token::new(TokenType::Minus, "-".to_string());
    assert_eq!(token1, token2);
}

#[test]
fn test_token_inequality() {
    let token1 = Token::new(TokenType::Star, "*".to_string());
    let token2 = Token::new(TokenType::Slash, "/".to_string());
    assert_ne!(token1, token2);
}

#[test]
fn test_token_to_string() {
    let token = Token::new(TokenType::Identifier, "variable".to_string());
    assert_eq!(token.to_string(), "variable");
}

#[test]
fn test_token_types() {
    let token = Token::new(TokenType::Equal, "=".to_string());
    assert_eq!(token.token_type, TokenType::Equal);

    let token = Token::new(TokenType::NotEqual, "<>".to_string());
    assert_eq!(token.token_type, TokenType::NotEqual);

    let token = Token::new(TokenType::Greater, ">".to_string());
    assert_eq!(token.token_type, TokenType::Greater);

    let token = Token::new(TokenType::Less, "<".to_string());
    assert_eq!(token.token_type, TokenType::Less);

    let token = Token::new(TokenType::String, "string".to_string());
    assert_eq!(token.token_type, TokenType::String);

    let token = Token::new(TokenType::Float, "3.14".to_string());
    assert_eq!(token.token_type, TokenType::Float);

    let token = Token::new(TokenType::Integer, "42".to_string());
    assert_eq!(token.token_type, TokenType::Integer);
}