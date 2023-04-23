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
            '%' => Token::Modulus,
            '=' => Token::Equal,
            ':' => Token::Colon,
            '0'..='9' => {
                let mut string_num = String::with_capacity(32);
                string_num.push(ch);

                for (_, num_char) in chars.take_while_ref(|(_, it)| it.is_numeric() || *it == '.') {
                    string_num.push(num_char);
                }

                if string_num.contains('.') {
                    match string_num.parse::<f32>() {
                        Ok(flt) => Token::Float(flt),
                        Err(_err) => return Err(format!("Unable to parse float [{}]", string_num))
                    }
                } else {
                    match string_num.parse::<i32>() {
                        Ok(integer) => Token::Integer(integer),
                        Err(_err) => return Err(format!("Unable to parse integer [{}]", string_num)) // Is this even possible to hit?
                    }
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
        "float" => Token::Keyword(Keyword::Float),
        "let" => Token::Keyword(Keyword::Let),
        _ => Token::Identifier(string)
    }
}
