use itertools::Itertools;

mod tests;
pub mod types;
use types::*;
 
pub fn scanner(code: &str) -> Result<Vec<Token>, String> {
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
                        '0'..='9' | '.' => {
                            string_num.push(*num_ch);
                            chars.next();
                        },
                        _ => break
                    }
                }

                match string_num.parse::<i32>() {
                    Ok(integer) => Token::Integer(integer),
                    Err(_err) => return Err(format!("Unable to parse integer [{}]", string_num))
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
            _ => return Err("Invalid Character Found".to_string())
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
