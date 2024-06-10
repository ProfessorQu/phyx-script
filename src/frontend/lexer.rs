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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VarType {
    Shape,
    Size,
    Stroke,
    Speed,
    Gravity,
    Color
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Number(String),
    Identifier(String),

    BinaryOperator(String),
    Equals,
    OpenParen,
    CloseParen,
    Semicolon,
    Comma,
    Eof,

    Shape(ShapeType),
    Var(VarType)
}

static KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
    "circle" => Token::Shape(ShapeType::Circle),
    "square" => Token::Shape(ShapeType::Square),
    "ring" => Token::Shape(ShapeType::Ring),

    "size" => Token::Var(VarType::Size),
    "stroke" => Token::Var(VarType::Stroke),
    "speed" => Token::Var(VarType::Speed),
    "gravity" => Token::Var(VarType::Gravity),
    "color" => Token::Var(VarType::Color),
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
            ',' => tokens.push(Token::Comma),
            '+' | '-' | '*' | '/' => tokens.push(Token::BinaryOperator(c.to_string())),
            _ if c.is_numeric() => {
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
