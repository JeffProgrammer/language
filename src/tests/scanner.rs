use crate::*;

#[test]
fn test_scan_integer() {
    let code = "12";

    let tokens: Vec<Token> = scanner(code).unwrap();
    let expected_tokens: Vec<Token> = vec![Token::Integer(12)];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_scan_float() {
    let code = "12.0";

    let tokens: Vec<Token> = scanner(code).unwrap();
    let expected_tokens: Vec<Token> = vec![Token::Float(12.0)];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_scan_invalid_float() {
    let code = "12.0.0";

    let message: String = scanner(code).unwrap_err();
    let expected_message = "Unable to parse float [12.0.0]";

    assert_eq!(message, expected_message);
}

#[test]
fn test_scan_integer_plus_integer() {
    let code = "12 + 13";

    let tokens: Vec<Token> = scanner(code).unwrap();
    let expected_tokens: Vec<Token> = vec![Token::Integer(12), Token::Plus, Token::Integer(13)];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_scan_integer_minus_integer() {
    let code = "12 - 13";

    let tokens: Vec<Token> = scanner(code).unwrap();
    let expected_tokens: Vec<Token> = vec![Token::Integer(12), Token::Minus, Token::Integer(13)];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_scan_integer_multiply_integer() {
    let code = "12 * 13";

    let tokens: Vec<Token> = scanner(code).unwrap();
    let expected_tokens: Vec<Token> = vec![Token::Integer(12), Token::Multiply, Token::Integer(13)];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_scan_integer_divide_integer() {
    let code = "12/13";

    let tokens: Vec<Token> = scanner(code).unwrap();
    let expected_tokens: Vec<Token> = vec![Token::Integer(12), Token::Divide, Token::Integer(13)];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_scan_integer_mod_integer() {
    let code = "12 % 13";

    let tokens: Vec<Token> = scanner(code).unwrap();
    let expected_tokens: Vec<Token> = vec![Token::Integer(12), Token::Modulus, Token::Integer(13)];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_let_assignment() {
    let code = "let x: int = 11;";

    let tokens: Vec<Token> = scanner(code).unwrap();

    let expected_tokens: Vec<Token> = vec![
        Token::Keyword(Keyword::Let), 
        Token::Identifier("x".to_string()),
        Token::Colon,
        Token::Keyword(Keyword::Int),
        Token::Equal, 
        Token::Integer(11)
    ];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_multiple_let_statements() {
    let code = "
        let x: int = 1;
        let y: float = 2.0;
    ";

    let tokens = scanner(code).unwrap();

    let expected_tokens = vec![
        Token::Keyword(Keyword::Let), 
        Token::Identifier("x".to_string()),
        Token::Colon,
        Token::Keyword(Keyword::Int),
        Token::Equal, 
        Token::Integer(1),
        Token::Keyword(Keyword::Let), 
        Token::Identifier("y".to_string()),
        Token::Colon,
        Token::Keyword(Keyword::Float),
        Token::Equal, 
        Token::Float(2.0)
    ];

    assert_eq!(tokens, expected_tokens);
}
