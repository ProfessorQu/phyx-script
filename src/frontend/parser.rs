use std::collections::HashMap;

use super::{ast::Statement, lexer::{tokenize, Token}, ShapeType};

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

    fn expect(&mut self, expected: Token, error: String) -> Result<Token, String> {
        match self.eat() {
            token if token != expected => Err(error),
            token => Ok(token)
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
            Token::Let => self.parse_var_declaration(),
            Token::Fn => self.parse_function_declaration(),
            Token::For => self.parse_for_loop(),
            _ => self.parse_expr()
        }
    }

    fn parse_var_declaration(&mut self) -> Result<Statement, String>{
        self.eat();
        let identifier = match self.eat() {
            Token::Identifier(name) => name,
            _ => return Err("Token after let isn't an identifier".to_string())
        };

        self.expect(Token::Equals, "Variable isn't set with equals sign".to_string())?;

        let value = self.parse_expr()?;
        let declaration = Statement::VarDeclaration { identifier, value: Box::new(value) };

        self.expect(Token::Semicolon, "Variable declaration isn't stopped with a semicolon".to_string())?;

        Ok(declaration)
    }

    fn parse_function_declaration(&mut self) -> Result<Statement, String> {
        self.eat();
        let name = match self.eat() {
            Token::Identifier(name) => name,
            token => return Err(format!("Invalid token '{:?}'", token))
        };

        let args = self.parse_args()?;
        let parameters: Vec<String> = args.iter().map(|arg| {
            match arg {
                Statement::Identifier(name) => name.clone(),
                _ => panic!("Argument '{:?}' is not an identifier", arg)
            }
        }).collect();

        self.expect(Token::OpenBracket, "Expected open bracket after function declaration".to_string())?;

        let mut body = vec![];
        while self.at() != Token::CloseBracket && self.not_eof() {
            body.push(self.parse_statement()?);
        }

        self.expect(Token::CloseBracket, "Expected close bracket after function body".to_string())?;

        Ok(Statement::FunctionDeclaration { name, parameters, body })
    }

    fn parse_for_loop(&mut self) -> Result<Statement, String> {
        self.eat();
        let loop_var = match self.eat() {
            Token::Identifier(name) => name,
            token => return Err(format!("Expected identifier, got: {:?}", token))
        };

        self.expect(Token::In, "Expected 'in' after for loop".to_string())?;

        let range = self.parse_statement()?;

        self.expect(Token::OpenBracket, "Expected open bracket after function declaration".to_string())?;

        let mut body = vec![];
        while self.at() != Token::CloseBracket && self.not_eof() {
            body.push(self.parse_statement()?);
        }

        self.expect(Token::CloseBracket, "Expected close bracket after function body".to_string())?;

        Ok(Statement::ForLoop { loop_var, range: Box::new(range), body })
    }
    
    fn parse_expr(&mut self) -> Result<Statement, String> {
        self.parse_assignment_expr()
    }

    fn parse_assignment_expr(&mut self) -> Result<Statement, String> {
        let left = self.parse_additive_expr()?;

        if self.at() == Token::Equals {
            self.eat();
            let value = self.parse_assignment_expr()?;

            let assignment = Statement::AssignmentExpr { assignee: Box::new(left), value: Box::new(value) };

            Ok(assignment)
        } else {
            Ok(left)
        }
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
        let mut left = self.parse_call_member_expr()?;

        while let Token::BinaryOperator(operator) = self.at() {
            if operator != "*" && operator != "/" {
                break
            }

            self.eat();
            let right = self.parse_call_member_expr()?;

            left = Statement::BinaryExpr {
                left: Box::new(left),
                right: Box::new(right),
                operator
            };
        }

        Ok(left)
    }

    fn parse_call_member_expr(&mut self) -> Result<Statement, String> {
        let member = self.parse_member_expr()?;

        if self.at() == Token::OpenParen {
            self.parse_call_expr(member)
        } else {
            Ok(member)
        }
    }

    fn parse_call_expr(&mut self, caller: Statement) -> Result<Statement, String> {
        let mut call_expr = Statement::CallExpr {
            args: self.parse_args()?,
            caller: Box::new(caller)
        };

        if self.at() == Token::OpenParen {
            call_expr = self.parse_call_expr(call_expr)?;
        }

        Ok(call_expr)
    }

    fn parse_args(&mut self) -> Result<Vec<Statement>, String> {
        self.expect(Token::OpenParen, "Expected open parenthesis".to_string())?;

        let args = match self.at() {
            Token::CloseParen => vec![],
            _ => self.parse_arguments_list()?
        };

        self.expect(Token::CloseParen, "Missing closing parenthesis".to_string())?;

        Ok(args)
    }

    fn parse_arguments_list(&mut self) -> Result<Vec<Statement>, String> {
        let mut args = vec![self.parse_assignment_expr()?];

        while self.at() == Token::Comma {
            self.eat();
            args.push(self.parse_assignment_expr()?);
        }

        Ok(args)
    }

    fn parse_member_expr(&mut self) -> Result<Statement, String> {
        let mut object = self.parse_primary_expr()?;

        while self.at() == Token::Dot {
            self.eat();
            let property = self.parse_primary_expr()?;
            match property {
                Statement::Identifier(_) => (),
                statement => return Err(format!("Invalid statement '{:?}'", statement))
            }

            object = Statement::MemberExpr {
                object: Box::new(object),
                property: Box::new(property)
            };
        }

        Ok(object)
    }

    fn parse_primary_expr(&mut self) -> Result<Statement, String> {
        let token = self.eat();

        match token {
            Token::Shape(shape) => Ok(self.parse_shape(shape)?),
            Token::Identifier(value) => Ok(Statement::Identifier(value)),
            Token::Number(number) => Ok(Statement::NumericLiteral(number.parse().expect("Failed to parse"))),
            Token::UnaryOperator(operator) => Ok(self.parse_unary_expr(operator)?),
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

    fn parse_shape(&mut self, shape: ShapeType) -> Result<Statement, String> {
        self.expect(Token::OpenBracket, format!("The shape '{:?}' has to be opened with an open bracket", shape))?;

        let mut map = HashMap::new();
        map.insert("shape".to_string(), Statement::Shape(shape));

        while self.at() != Token::CloseBracket {
            let key = match self.eat() {
                Token::Identifier(name) => name,
                token => return Err(format!("Invalid token '{:?}', should be identifier", token)),
            };

            match self.at() {
                Token::Colon => {
                    self.eat();
                    let value = self.parse_statement()?;
                    map.insert(key.clone(), value);

                    if self.at() != Token::CloseBracket {
                        self.expect(Token::Comma, format!("Forgot to close '{:?}' with a comma", key))?;
                    }
                }
                Token::Comma => {
                    self.eat();
                    map.insert(key.clone(), Statement::Identifier(key));
                }
                Token::CloseBracket => {
                    map.insert(key.clone(), Statement::Identifier(key));
                    break
                }
                token => return Err(format!("Invalid token '{:?}', should be ':', ',' or '}}'", token))
            }
        }

        self.expect(Token::CloseBracket, format!("The shape {:?} wasn't closed with a close bracket", shape))?;

        Ok(Statement::Object(map))
    }

    fn parse_unary_expr(&mut self, operator: String) -> Result<Statement, String> {
        let value = self.parse_expr()?;

        Ok(Statement::UnaryExpr { value: Box::new(value), operator })
    }

}
