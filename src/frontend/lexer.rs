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

    BinaryOperator(String),
    BooleanOperator(String),
    UnaryOperator(String),

    Equals,
    CompoundEquals(String),

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
    Else,
    While,

    Object
}

static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "let" => Token::Let,
    "fn" => Token::Fn,
    "for" => Token::For,
    "in" => Token::In,
    "if" => Token::If,
    "else" => Token::Else,
    "while" => Token::While,

    "object" => Token::Object
};

/// Convert a string of code into a vector of Tokens
pub fn tokenize(source_code: String) -> Vec<Token> {
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
            '+' | '*' | '%' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::CompoundEquals(c.to_string()))
                } else {
                    tokens.push(Token::BinaryOperator(c.to_string()))
                }
            }
            '-' => {
                match chars.peek() {
                    Some(c2) if c2.is_whitespace() => tokens.push(Token::BinaryOperator(c.to_string())),
                    Some('=') => {
                        chars.next();
                        tokens.push(Token::CompoundEquals(c.to_string()))
                    }
                    _ => tokens.push(Token::UnaryOperator(c.to_string()))
                }
            }
            '|' => {
                match chars.peek() {
                    Some('|') => {
                        chars.next();
                        tokens.push(Token::BooleanOperator("||".to_string()));
                    }
                    c2 => panic!("Expected '|' after '|' got '{:?}'", c2)
                }
            }
            '&' => {
                match chars.peek() {
                    Some('&') => {
                        chars.next();
                        tokens.push(Token::BooleanOperator("&&".to_string()));
                    }
                    c2 => panic!("Expected '&' after '&' got '{:?}'", c2)
                }
            }
            '/' => {
                match chars.peek() {
                    Some('/') =>  {
                        for c2 in chars.by_ref() {
                            if c2 == '\n' || c2 == '\r' {
                                break
                            }
                        }
                    }
                    Some('*') => {
                        let mut star_found = false;
                        for c2 in chars.by_ref() {
                            if c2 == '*' {
                                star_found = true;
                            } else if c2 == '/' && star_found {
                                break
                            } else {
                                star_found = false;
                            }
                        }
                    }
                    _ => tokens.push(Token::BinaryOperator(c.to_string()))
                }
            }
            '!' => {
                match chars.peek() {
                    Some('=') => {
                        chars.next();
                        tokens.push(Token::Comparison("!=".to_string()))
                    },
                    _ => tokens.push(Token::UnaryOperator("!".to_string()))
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
                let mut num_string = c.to_string();
                let mut decimal_in_string = false;
                while let Some(&next) = chars.peek() {
                    if next.is_numeric() {
                        chars.next();
                        num_string.push(next);
                    } else if next == '.' {
                        if decimal_in_string {
                            panic!("'.' already used in this number")
                        }

                        chars.next();
                        num_string.push(next);
                        decimal_in_string = true;
                    } else {
                        break
                    }
                }

                tokens.push(Token::Number(num_string))
            }
            _ if c.is_alphabetic() || c == '_' => {
                let mut id_string = c.to_string();
                while let Some(&next) = chars.peek() {
                    if next.is_alphabetic() || next.is_numeric() || next == '_' || next == '#' {
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
            _ => panic!("Unknown token: {:?}", c)
        }
    }

    tokens.push(Token::Eof);
    tokens
}
