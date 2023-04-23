use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Int,
    Let
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    Colon,
    Keyword(Keyword),
    Identifier(String),
    Integer(i32)
}
 
pub fn scanner(code: &str) -> Result<Vec<Token>, &'static str> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = code.char_indices().peekable();

    while let Some((_, ch)) = chars.next() {

        let token = match ch {
            ';' | ' ' | '\n' | '\r' => continue,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '=' => Token::Equal,
            ':' => Token::Colon,
            '0'..='9' => {
                let mut string_num = String::with_capacity(32);
                string_num.push(ch);

                while let Some((_num_pos, num_ch)) = chars.peek() {
                    match *num_ch {
                        '0'..='9' => {
                            string_num.push(*num_ch);
                            chars.next();
                        },
                        _ => break
                    }
                }

                match string_num.parse::<i32>() {
                    Ok(integer) => Token::Integer(integer),
                    Err(_err) => return Err("Unable to parse integer")
                }
            },
            'a'..='z' => {
                let mut string = String::with_capacity(32);
                string.push(ch);

                for (_, identifier_chr) in chars.take_while_ref(|(_, str_ch) | str_ch.is_alphanumeric()) {
                    string.push(identifier_chr);
                }

                match_identifier(string)
            }
            _ => return Err("Invalid Character Found")
        };

        tokens.push(token);
    }

    return Ok(tokens);
}

fn match_identifier(string: String) -> Token {
    return match string.as_str() {
        "int" => Token::Keyword(Keyword::Int),
        "let" => Token::Keyword(Keyword::Let),
        _ => Token::Identifier(string)
    }
}

#[test]
fn test_scan_integer() {
    let tokens: Vec<Token> = scanner("12").unwrap();
    let expected_tokens: Vec<Token> = vec![Token::Integer(12)];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_scan_integer_plus_integer() {
    let tokens: Vec<Token> = scanner("12+13").unwrap();
    let expected_tokens: Vec<Token> = vec![Token::Integer(12), Token::Plus, Token::Integer(13)];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_scan_integer_minus_integer() {
    let tokens: Vec<Token> = scanner("12-13").unwrap();
    let expected_tokens: Vec<Token> = vec![Token::Integer(12), Token::Minus, Token::Integer(13)];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_scan_integer_multiply_integer() {
    let tokens: Vec<Token> = scanner("12*13").unwrap();
    let expected_tokens: Vec<Token> = vec![Token::Integer(12), Token::Multiply, Token::Integer(13)];

    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_scan_integer_divide_integer() {
    let tokens: Vec<Token> = scanner("12/13").unwrap();
    let expected_tokens: Vec<Token> = vec![Token::Integer(12), Token::Divide, Token::Integer(13)];

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
        let y: int = 2;
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
        Token::Keyword(Keyword::Int),
        Token::Equal, 
        Token::Integer(2)
    ];

    assert_eq!(tokens, expected_tokens);
}
