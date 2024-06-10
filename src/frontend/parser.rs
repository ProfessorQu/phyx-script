use std::collections::HashMap;

use super::{ast::Statement, lexer::{tokenize, Token, VarType}, ShapeType};

pub struct Parser {
    tokens: Vec<Token>
}

impl Parser {
    pub fn new() -> Self {
        Self {
            tokens: vec![]
        }
    }

    fn not_eof(&self) -> bool {
        if let Some(Token::Eof) = self.tokens.first() {
            return false;
        }
        true
    }

    fn at(&self) -> Token {
        self.tokens.first().expect("Called 'at' with empty 'tokens' array").clone()
    }

    fn eat(&mut self) -> Token {
        self.tokens.remove(0)
    }

    fn expect(&mut self, expected: Token, error: String) -> Result<(), String> {
        if self.eat() != expected {
            Err(error)
        } else {
            Ok(())
        }
    }

    pub fn produce_ast(&mut self, source_code: String) -> Result<Statement, String> {
        self.tokens = tokenize(source_code)?;
        let mut body = vec![];

        while self.not_eof() {
            body.push(self.parse_statement().expect("Invalid statement"));
        }

        Ok(Statement::Program{ body })
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.at() {
            Token::Shape(shape) => self.parse_shape(shape),
            _ => self.parse_expr()
        }
    }

    fn parse_shape(&mut self, shape: ShapeType) -> Result<Statement, String> {
        self.eat();
        self.expect(Token::OpenParen, format!("The shape '{:?} have to be opened with parentheses", shape))?;

        let mut map = HashMap::new();
        map.insert(VarType::Shape, Statement::Shape(shape));

        while self.at() != Token::CloseParen {
            let token = self.eat();
            match token {
                Token::Var(name) => {
                    self.expect(Token::Equals, format!("The variable {:?} wasn't set", name))?;

                    let value = self.parse_expr()?;
                    map.insert(name, value);

                    self.expect(Token::Semicolon, format!("The variable {:?} wasn't closed with a semicolon", name))?;
                }
                _ => return Err(format!("Token '{:?}' is invalid in this context", token))
            }
        }

        self.expect(Token::CloseParen, format!("The shape {:?} wasn't closed with a close parentheses", shape))?;

        Ok(Statement::Element(map))
    }

    fn parse_expr(&mut self) -> Result<Statement, String> {
        self.parse_additive_expr()
    }

    fn parse_additive_expr(&mut self) -> Result<Statement, String> {
        let mut left = self.parse_multiplicative_expr()?;

        while let Token::BinaryOperator(operator) = self.at() {
            if operator != "+" && operator != "-" {
                break
            }

            self.eat();
            let right = self.parse_multiplicative_expr()?;

            left = Statement::BinaryExpr {
                left: Box::new(left),
                right: Box::new(right),
                operator
            };
        }

        Ok(left)
    }

    fn parse_multiplicative_expr(&mut self) -> Result<Statement, String> {
        let mut left = self.parse_primary_expr()?;

        while let Token::BinaryOperator(operator) = self.at() {
            if operator != "*" && operator != "/" {
                break
            }

            self.eat();
            let right = self.parse_primary_expr()?;

            left = Statement::BinaryExpr {
                left: Box::new(left),
                right: Box::new(right),
                operator
            };
        }

        Ok(left)
    }

    fn parse_primary_expr(&mut self) -> Result<Statement, String> {
        let token = self.eat();

        match token {
            Token::Identifier(value) => Ok(Statement::Identifier(value)),
            Token::Number(number) => Ok(Statement::NumericLiteral(number.parse().expect("Failed to parse"))),
            Token::OpenParen => {
                let value = self.parse_expr()?;
                if let Token::CloseParen = self.eat() {
                    Ok(value)
                } else {
                    Err("Opened parentheses isn't closed!".to_string())
                }
            }
            _ => Err(format!("Unexpected token found during parsing: {:?}", token))
        }
    }
}
