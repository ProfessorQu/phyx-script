use std::iter::Peekable;

use phf::phf_map;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShapeType {
    Circle,
    Rect,
    Ring
}

impl TryFrom<String> for ShapeType {
    type Error = String;
    
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "circle" => Ok(Self::Circle),
            "rect" => Ok(Self::Rect),
            "ring" => Ok(Self::Ring),
            _ => Err(format!("{:?} is not a valid shape", value))
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Number(String),
    Identifier(String),

    Object,
    BinaryOperator(String),
    UnaryOperator(String),

    Equals,

    OpenParen,
    CloseParen,

    OpenBracket,
    CloseBracket,

    Semicolon,
    Comma,
    Colon,
    Dot,

    Comparison(String),

    Eof,

    Let,
    Fn,
    For,
    In,
    If,
    Else
}

static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "let" => Token::Let,
    "fn" => Token::Fn,
    "for" => Token::For,
    "in" => Token::In,
    "if" => Token::If,
    "else" => Token::Else,

    "object" => Token::Object
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
            ';' => tokens.push(Token::Semicolon),
            ':' => tokens.push(Token::Colon),
            ',' => tokens.push(Token::Comma),
            '.' => tokens.push(Token::Dot),
            '+' | '*' | '/' | '%' => tokens.push(Token::BinaryOperator(c.to_string())),
            '-' => {
                if let Some(c2) = chars.peek() {
                    if c2.is_whitespace() {
                        tokens.push(Token::BinaryOperator(c.to_string()));
                    } else {
                        tokens.push(Token::UnaryOperator(c.to_string()));
                    }
                }
            }
            '=' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Comparison("==".to_string()))
                } else {
                    tokens.push(Token::Equals)
                }
            }
            '>' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Comparison(">=".to_string()))
                } else {
                    tokens.push(Token::Comparison(">".to_string()))
                }
            }
            '<' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Comparison("<=".to_string()))
                } else {
                    tokens.push(Token::Comparison("<".to_string()))
                }
            }
            _ if c.is_numeric() => {
                tokens.push(Token::Number(get_number_string(c, &mut chars)?))
            }
            _ if c.is_alphabetic() || c == '_' => {
                let mut id_string = c.to_string();
                while let Some(&next) = chars.peek() {
                    if next.is_alphabetic() || next == '_' {
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
