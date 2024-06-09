use phf::phf_map;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Number(String),
    Identifier(String),

    BinaryOperator(String),
    Equals,
    OpenParen,
    CloseParen,
    Semicolon,
    Eof,

    Shape(&'static str),
    Var(&'static str)
}

static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "circle" => Token::Shape("circle"),
    "square" => Token::Shape("square"),

    "size" => Token::Var("size"),
    "speed" => Token::Var("speed"),
    "gravity" => Token::Var("gravity")
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

                tokens.push(Token::Number(num_string))
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

    tokens.push(Token::Eof);
    Ok(tokens)
}
