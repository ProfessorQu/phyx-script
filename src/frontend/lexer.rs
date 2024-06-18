use std::iter::Peekable;

use phf::phf_map;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShapeType {
    Circle,
    Square,
    Ring
}

impl TryFrom<String> for ShapeType {
    type Error = String;
    
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "circle" => Ok(Self::Circle),
            "square" => Ok(Self::Square),
            "ring" => Ok(Self::Ring),
            _ => Err(format!("{:?} is not a valid shape", value))
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Number(String),
    Identifier(String),

    Shape(ShapeType),
    BinaryOperator(String),

    Equals,

    OpenParen,
    CloseParen,

    OpenBracket,
    CloseBracket,

    Semicolon,
    Comma,
    Colon,

    Eof,

    Let
}

static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "let" => Token::Let,

    "circle" => Token::Shape(ShapeType::Circle),
    "square" => Token::Shape(ShapeType::Square),
    "ring" => Token::Shape(ShapeType::Ring)
};

fn get_number_string(c: char, chars: &mut Peekable<std::str::Chars<'_>>) -> Result<String, String> {
    let mut num_string = c.to_string();
    let mut decimal_in_string = false;
    while let Some(&next) = chars.peek() {
        if next.is_numeric() {
            chars.next();
            num_string.push(next);
        } else if next == '.' {
            if decimal_in_string {
                return Err("'.' already used in this number".to_string())
            }

            chars.next();
            num_string.push(next);
            decimal_in_string = true;
        } else {
            break
        }
    }

    Ok(num_string)
}

pub fn tokenize(source_code: String) -> Result<Vec<Token>, String> {
    let mut tokens = vec![];
    let mut chars = source_code.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            '{' => tokens.push(Token::OpenBracket),
            '}' => tokens.push(Token::CloseBracket),
            '=' => tokens.push(Token::Equals),
            ';' => tokens.push(Token::Semicolon),
            ':' => tokens.push(Token::Colon),
            ',' => tokens.push(Token::Comma),
            '+' | '*' | '/' => tokens.push(Token::BinaryOperator(c.to_string())),
            '-' => {
                let mut number = false;
                while let Some(c2) = chars.peek() {
                    if c2.is_numeric() {
                        tokens.push(Token::Number(get_number_string(c, &mut chars)?));
                        number = true;
                    } else {
                        break
                    }
                }

                if !number {
                    tokens.push(Token::BinaryOperator(c.to_string()));
                }
            }
            _ if c.is_numeric() => {
                tokens.push(Token::Number(get_number_string(c, &mut chars)?))
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
