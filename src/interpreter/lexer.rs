use phf::phf_map;

#[derive(Clone, Debug)]
pub enum Token {
    Number(i32),
    Identifier(String),
    BinaryOperator(String),
    Equals,
    OpenParen,
    CloseParen,
    Semicolon,
    Let
}

static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "let" => Token::Let
};

pub fn tokenize(source_code: String) -> Result<Vec<Token>, String> {
    let mut tokens = vec![];
    let mut chars = source_code.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            '=' => tokens.push(Token::Equals),
            ';' => tokens.push(Token::Semicolon),
            '+' | '-' | '*' | '/' => tokens.push(Token::BinaryOperator(c.to_string())),
            _ if c.is_numeric() => {
                let mut num_string = c.to_string();
                while let Some(&next) = chars.peek() {
                    if next.is_numeric() {
                        chars.next();
                        num_string.push(next);
                    } else {
                        break
                    }
                }

                if let Ok(number) = num_string.parse() {
                    tokens.push(Token::Number(number))
                }
            }
            _ if c.is_alphabetic() => {
                let mut id_string = c.to_string();
                while let Some(&next) = chars.peek() {
                    if next.is_alphabetic() {
                        chars.next();
                        id_string.push(next);
                    } else {
                        break
                    }
                }

                if let Some(value) = KEYWORDS.get(&id_string) {
                    tokens.push(value.clone());
                } else {
                    tokens.push(Token::Identifier(id_string));
                }
            }
            _ if c.is_whitespace() => (),
            _ => return Err(format!("Unknown token: {:?}", c))
        }
    }

    Ok(tokens)
}
