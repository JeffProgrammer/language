use crate::types::{ASTNode, Keyword, Token, TokenType, VariableType};
use std::{iter::Peekable, slice::IterMut};

pub fn parse(mut tokens: Vec<Token>) -> Result<Vec<ASTNode>, String> {
    let mut vec = vec![];

    let mut token_iterator = tokens.iter_mut().peekable();

    while token_iterator.peek_mut() != None {
        vec.push(statement(&mut token_iterator)?);
    }

    return Ok(vec);
}

fn statement(token_iterator: &mut Peekable<IterMut<Token>>) -> Result<ASTNode, String> {
    let token_val = token_iterator.next().unwrap();

    match token_val.token {
        TokenType::Keyword(Keyword::Let) => {
            return variable_assignment(token_iterator);
        }
        _ => {
            return Err(format!(
                "Expected Statement on line {}, but unrecognized token: {:?}",
                token_val.line_number, token_val.token
            ));
        }
    }
}

fn variable_assignment(token_iterator: &mut Peekable<IterMut<Token>>) -> Result<ASTNode, String> {
    let identifier_token = token_iterator.next().unwrap();

    if let TokenType::Identifier(identifier) = &identifier_token.token {
        let tok = token_iterator.peek().unwrap();

        let explicit_variable_type = if tok.token == TokenType::Colon {
            token_iterator.next();

            let var_token = token_iterator.peek().unwrap();
            let var_type = find_type(var_token.token.clone());

            if var_type == None {
                return Err(format!(
                    "Line [{}]: Invalid Type {:?} on Variable {} Declaration",
                    var_token.line_number, var_token.token, identifier
                ));
            }

            token_iterator.next();
            var_type
        } else {
            None
        };

        let mut assignment_expression: Option<Box<ASTNode>> = None;

        if let Some(Token {
            token: TokenType::Equal,
            ..
        }) = token_iterator.peek()
        {
            token_iterator.next();
            assignment_expression = Some(parse_expression(token_iterator)?);
        }

        return Ok(ASTNode::VariableAssignment {
            name: identifier.to_string(),
            var_type: explicit_variable_type,
            expression: assignment_expression,
        });
    }

    return Err("".to_string());
}

fn parse_expression(token_iterator: &mut Peekable<IterMut<Token>>) -> Result<Box<ASTNode>, String> {
    return Err("Not Implemented".to_string());
}

fn find_type(token: TokenType) -> Option<VariableType> {
    return match token {
        TokenType::Keyword(Keyword::Int) => Some(VariableType::Int),
        TokenType::Keyword(Keyword::Float) => Some(VariableType::Float),
        _ => None,
    };
}
