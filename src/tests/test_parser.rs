#[allow(unused_imports)]
use crate::parse::*;
#[allow(unused_imports)]
use crate::types::*;

#[test]
fn test_parse_int_declaration() {
    let tokens = vec![
        Token {
            line_number: 1,
            token: TokenType::Keyword(Keyword::Let),
        },
        Token {
            line_number: 1,
            token: TokenType::Identifier("x".to_string()),
        },
        Token {
            line_number: 1,
            token: TokenType::Colon,
        },
        Token {
            line_number: 1,
            token: TokenType::Keyword(Keyword::Int),
        },
    ];

    let ast = parse(tokens).unwrap();

    assert_eq!(
        ast,
        vec![ASTNode::VariableAssignment {
            name: "x".to_string(),
            var_type: Some(VariableType::Int),
            expression: None
        }]
    )
}
