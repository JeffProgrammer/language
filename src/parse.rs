use crate::types::{Keyword, NodeType, Token, TokenType, TreeNode, VariableType};
use std::{iter::Peekable, slice::IterMut};

pub fn parse(mut tokens: Vec<Token>) -> Result<Vec<TreeNode>, String> {
    let mut vec = vec![];

    let mut token_iterator = tokens.iter_mut().peekable();

    while token_iterator.peek_mut() != None {
        vec.push(statement(&mut token_iterator)?);
    }

    return Ok(vec);
}

fn statement(token_iterator: &mut Peekable<IterMut<Token>>) -> Result<TreeNode, String> {
    let mut tree = TreeNode {
        leaf: NodeType::StatementOp,
        left_branch: None,
        right_branch: None,
    };

    let token_val = token_iterator.next().unwrap();

    match token_val.token {
        TokenType::Keyword(Keyword::Let) => {
            tree.left_branch = Some(variable_assignment(token_iterator)?);
        }
        _ => {}
    }

    return Ok(tree);
}

fn variable_assignment(
    token_iterator: &mut Peekable<IterMut<Token>>,
) -> Result<Box<TreeNode>, String> {
    let identifier_token = token_iterator.next().unwrap();

    if let TokenType::Identifier(identifier) = &identifier_token.token {
        let tok = token_iterator.peek();
        let type_token = match tok {
            None => None,
            _ => {
                let variable_type = find_type(tok.unwrap().token.clone());
                token_iterator.next();
                variable_type
            }
        };

        let mut assignment_expression: Option<Box<TreeNode>> = None;

        if let Some(Token {
            token: TokenType::Equal,
            ..
        }) = token_iterator.peek()
        {
            token_iterator.next();
            assignment_expression = Some(parse_expression(token_iterator)?);
        }

        return Ok(Box::new(TreeNode {
            leaf: NodeType::VariableAssignment(identifier.clone(), type_token),
            left_branch: assignment_expression,
            right_branch: None,
        }));
    }

    return Err("".to_string());
}

fn parse_expression(
    token_iterator: &mut Peekable<IterMut<Token>>,
) -> Result<Box<TreeNode>, String> {
    return Err("Not Implemented".to_string());
}

fn find_type(token: TokenType) -> Option<VariableType> {
    return match token {
        TokenType::Keyword(Keyword::Int) => Some(VariableType::Int),
        TokenType::Keyword(Keyword::Float) => Some(VariableType::Float),
        _ => None,
    };
}
