use itertools::Itertools;

mod tests;
pub mod types;
use types::*;
 
pub fn scanner(code: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = code.char_indices().peekable();
    let mut line_number = 1;

    while let Some((_, ch)) = chars.next() {

        let token = match ch {
            ';' | ' ' | '\r' | '\t' => continue,
            '\n' => {
                line_number += 1;
                continue;
            }
            '+' => Token { line_number: line_number, token: TokenType::Plus },
            '-' => Token { line_number: line_number, token: TokenType::Minus },
            '*' => Token { line_number: line_number, token: TokenType::Multiply },
            '/' => Token { line_number: line_number, token: TokenType::Divide },
            '%' => Token { line_number: line_number, token: TokenType::Modulus },
            '=' => Token { line_number: line_number, token: TokenType::Equal },
            ':' => Token { line_number: line_number, token: TokenType::Colon },
            '0'..='9' => {
                let mut string_num = String::with_capacity(32);
                string_num.push(ch);

                for (_, num_char) in chars.take_while_ref(|(_, it)| it.is_numeric() || *it == '.') {
                    string_num.push(num_char);
                }

                let token_type = if string_num.contains('.') {
                    match string_num.parse::<f32>() {
                        Ok(flt) => TokenType::Float(flt),
                        Err(_err) => return Err(format!("Unable to parse float [{}]", string_num))
                    }
                } else {
                    match string_num.parse::<i32>() {
                        Ok(integer) => TokenType::Integer(integer),
                        Err(_err) => return Err(format!("Unable to parse integer [{}]", string_num)) // Is this even possible to hit?
                    }
                };

                Token { line_number: line_number, token: token_type }
            },
            'a'..='z' => {
                let mut string = String::with_capacity(32);
                string.push(ch);

                for (_, identifier_chr) in chars.take_while_ref(|(_, str_ch) | str_ch.is_alphanumeric()) {
                    string.push(identifier_chr);
                }

                Token { line_number: line_number, token: match_identifier(string) }
            }
            _ => return Err("Invalid Character Found".to_string())
        };

        tokens.push(token);
    }

    return Ok(tokens);
}

fn match_identifier(string: String) -> TokenType {
    return match string.as_str() {
        "int" => TokenType::Keyword(Keyword::Int),
        "float" => TokenType::Keyword(Keyword::Float),
        "let" => TokenType::Keyword(Keyword::Let),
        _ => TokenType::Identifier(string)
    };
}
