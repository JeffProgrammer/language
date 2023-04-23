use crate::*;

#[test]
fn test_scan_integer() {
    let code = "12";

    let tokens: Vec<Token> = scanner(code).unwrap();
    let expected_tokens: Vec<Token> = vec![Token { line_number: 1, token: TokenType::Integer(12) }];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_scan_float() {
    let code = "12.0";

    let tokens: Vec<Token> = scanner(code).unwrap();
    let expected_tokens: Vec<Token> = vec![Token { line_number: 1, token: TokenType::Float(12.0) }];

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
    let expected_tokens: Vec<Token> = vec![
        Token { line_number: 1, token: TokenType::Integer(12) },
        Token { line_number: 1, token: TokenType::Plus },
        Token { line_number: 1, token: TokenType::Integer(13) },
    ];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_scan_integer_minus_integer() {
    let code = "12 - 13";

    let tokens: Vec<Token> = scanner(code).unwrap();
    let expected_tokens: Vec<Token> = vec![
        Token { line_number: 1, token: TokenType::Integer(12) },
        Token { line_number: 1, token: TokenType::Minus },
        Token { line_number: 1, token: TokenType::Integer(13) },
    ];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_scan_integer_multiply_integer() {
    let code = "12 * 13";

    let tokens: Vec<Token> = scanner(code).unwrap();
    let expected_tokens: Vec<Token> = vec![
        Token { line_number: 1, token: TokenType::Integer(12) },
        Token { line_number: 1, token: TokenType::Multiply },
        Token { line_number: 1, token: TokenType::Integer(13) },
    ];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_scan_integer_divide_integer() {
    let code = "12/13";

    let tokens: Vec<Token> = scanner(code).unwrap();
    let expected_tokens: Vec<Token> = vec![
        Token { line_number: 1, token: TokenType::Integer(12) },
        Token { line_number: 1, token: TokenType::Divide },
        Token { line_number: 1, token: TokenType::Integer(13) },
    ];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_scan_integer_mod_integer() {
    let code = "12 % 13";

    let tokens: Vec<Token> = scanner(code).unwrap();
    let expected_tokens: Vec<Token> = vec![
        Token { line_number: 1, token: TokenType::Integer(12) },
        Token { line_number: 1, token: TokenType::Modulus },
        Token { line_number: 1, token: TokenType::Integer(13) },
    ];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_let_assignment() {
    let code = "let x: int = 11;";

    let tokens: Vec<Token> = scanner(code).unwrap();

    let expected_tokens: Vec<Token> = vec![
        Token { line_number: 1, token: TokenType::Keyword(Keyword::Let) },
        Token { line_number: 1, token: TokenType::Identifier("x".to_string()) },
        Token { line_number: 1, token: TokenType::Colon },
        Token { line_number: 1, token: TokenType::Keyword(Keyword::Int) },
        Token { line_number: 1, token: TokenType::Equal },
        Token { line_number: 1, token: TokenType::Integer(11) },
    ];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_multiple_let_statements() {
    let code = "let x: int = 1;
        let y: float = 2.0;
    ";

    let tokens = scanner(code).unwrap();

    let expected_tokens: Vec<Token> = vec![
        Token { line_number: 1, token: TokenType::Keyword(Keyword::Let) },
        Token { line_number: 1, token: TokenType::Identifier("x".to_string()) },
        Token { line_number: 1, token: TokenType::Colon },
        Token { line_number: 1, token: TokenType::Keyword(Keyword::Int) },
        Token { line_number: 1, token: TokenType::Equal },
        Token { line_number: 1, token: TokenType::Integer(1) },
        Token { line_number: 2, token: TokenType::Keyword(Keyword::Let) },
        Token { line_number: 2, token: TokenType::Identifier("y".to_string()) },
        Token { line_number: 2, token: TokenType::Colon },
        Token { line_number: 2, token: TokenType::Keyword(Keyword::Float) },
        Token { line_number: 2, token: TokenType::Equal },
        Token { line_number: 2, token: TokenType::Float(2.0) },
    ];

    assert_eq!(tokens, expected_tokens);
}
