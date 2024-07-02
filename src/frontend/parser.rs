use std::collections::HashMap;

use super::{ast::Statement, lexer::{tokenize, Token}};

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

    fn expect(&mut self, expected: Token, error: String) -> Token {
        match self.eat() {
            token if token != expected => panic!("{}", error),
            token => token
        }
    }

    pub fn produce_ast(&mut self, source_code: String) -> Statement {
        self.tokens = tokenize(source_code);
        let mut body = vec![];

        while self.not_eof() {
            body.push(self.parse_statement());
        }

        Statement::Program{ body }
    }

    fn parse_statement(&mut self) -> Statement {
        match self.at() {
            Token::Let => self.parse_var_declaration(),
            Token::Fn => self.parse_function_declaration(),
            Token::For => self.parse_for_loop(),
            Token::If => self.parse_if_statement(),
            Token::While => self.parse_while_statement(),
            _ => self.parse_expr()
        }
    }

    fn parse_var_declaration(&mut self) -> Statement {
        self.eat();
        let identifier = match self.eat() {
            Token::Identifier(name) => name,
            token => panic!("Token {:?} after let isn't an identifier", token)
        };

        self.expect(Token::Equals, "Variable isn't set with equals sign".to_string());

        let value = self.parse_statement();

        Statement::VarDeclaration { identifier, value: Box::new(value) }
    }

    fn parse_function_declaration(&mut self) -> Statement {
        self.eat();
        let name = match self.eat() {
            Token::Identifier(name) => name,
            token => panic!("Invalid token '{:?}'", token)
        };

        let args = self.parse_args();
        let parameters: Vec<String> = args.iter().map(|arg| {
            match arg {
                Statement::Identifier(name) => name.clone(),
                _ => panic!("Argument '{:?}' is not an identifier", arg)
            }
        }).collect();

        self.expect(Token::OpenBracket, "Expected open bracket after function declaration".to_string());

        let mut body = vec![];
        while self.at() != Token::CloseBracket && self.not_eof() {
            body.push(self.parse_statement());
        }

        self.expect(Token::CloseBracket, "Expected close bracket after function body".to_string());

        Statement::FunctionDeclaration { name, parameters, body }
    }

    fn parse_for_loop(&mut self) -> Statement {
        self.eat();
        let loop_var = match self.eat() {
            Token::Identifier(name) => name,
            token => panic!("Expected identifier, got: {:?}", token)
        };

        self.expect(Token::In, "Expected 'in' after for loop".to_string());

        let range = self.parse_statement();

        self.expect(Token::OpenBracket, "Expected open bracket after for loop declaration".to_string());

        let mut body = vec![];
        while self.at() != Token::CloseBracket && self.not_eof() {
            body.push(self.parse_statement());
        }

        self.expect(Token::CloseBracket, "Expected close bracket after function body".to_string());

        Statement::ForLoop { loop_var, range: Box::new(range), body }
    }

    fn parse_if_statement(&mut self) -> Statement {
        self.eat();
        self.expect(Token::OpenParen, "Expected parentheses before condition".to_string());

        let condition = self.parse_expr();

        self.expect(Token::CloseParen, "Expected parentheses after condition".to_string());
        self.expect(Token::OpenBracket, "Expected open bracket before if block".to_string());

        let mut body = vec![];
        while self.at() != Token::CloseBracket && self.not_eof() {
            body.push(self.parse_statement());
        }

        self.expect(Token::CloseBracket, "Expected close bracket after if block".to_string());

        let mut else_body = vec![];
        if self.at() == Token::Else {
            self.eat();
            self.expect(Token::OpenBracket, "Expected open bracket before else block".to_string());

            while self.at() != Token::CloseBracket && self.not_eof() {
                else_body.push(self.parse_statement());
            }

            self.expect(Token::CloseBracket, "Expected close bracket after else block".to_string());
        }

        Statement::If { condition: Box::new(condition), body, else_body }
    }

    fn parse_while_statement(&mut self) -> Statement {
        self.eat();
        self.expect(Token::OpenParen, "Expected parentheses before condition".to_string());

        let condition = self.parse_expr();

        self.expect(Token::CloseParen, "Expected parentheses after condition".to_string());
        self.expect(Token::OpenBracket, "Expected open bracket before if block".to_string());

        let mut body = vec![];
        while self.at() != Token::CloseBracket && self.not_eof() {
            body.push(self.parse_statement());
        }

        self.expect(Token::CloseBracket, "Expected close bracket after if block".to_string());

        let mut else_body = vec![];
        if self.at() == Token::Else {
            self.eat();
            self.expect(Token::OpenBracket, "Expected open bracket before else block".to_string());

            while self.at() != Token::CloseBracket && self.not_eof() {
                else_body.push(self.parse_statement());
            }

            self.expect(Token::CloseBracket, "Expected close bracket after else block".to_string());
        }

        Statement::While { condition: Box::new(condition), body }
    }
    
    fn parse_expr(&mut self) -> Statement {
        self.parse_assignment_expr()
    }

    fn parse_assignment_expr(&mut self) -> Statement {
        let left = self.parse_comparison_expr();

        if self.at() == Token::Equals {
            self.eat();
            let value = self.parse_statement();

            Statement::AssignmentExpr { assignee: Box::new(left), value: Box::new(value) }
        } else {
            left
        }
    }

    fn parse_comparison_expr(&mut self) -> Statement {
        let mut left = self.parse_boolean_expr();

        while let Token::Comparison(operator) = self.at() {
            self.eat();
            let right = self.parse_boolean_expr();

            left = Statement::Comparison {
                left: Box::new(left),
                right: Box::new(right),
                operator
            };
        }

        left
    }

    fn parse_boolean_expr(&mut self) -> Statement {
        let mut left = self.parse_additive_expr();

        while let Token::BooleanOperator(operator) = self.at() {
            self.eat();

            let right = self.parse_additive_expr();

            left = Statement::BooleanExpr {
                left: Box::new(left),
                right: Box::new(right),
                operator
            }
        }

        left
    }

    fn parse_additive_expr(&mut self) -> Statement {
        let mut left = self.parse_multiplicative_expr();

        while let Token::BinaryOperator(operator) = self.at() {
            if operator != "+" && operator != "-" {
                break
            }

            self.eat();
            let right = self.parse_multiplicative_expr();

            left = Statement::BinaryExpr {
                left: Box::new(left),
                right: Box::new(right),
                operator
            };
        }

        left
    }

    fn parse_multiplicative_expr(&mut self) -> Statement {
        let mut left = self.parse_call_member_expr();

        while let Token::BinaryOperator(operator) = self.at() {
            if operator != "*" && operator != "/" && operator != "%" {
                break
            }

            self.eat();
            let right = self.parse_call_member_expr();

            left = Statement::BinaryExpr {
                left: Box::new(left),
                right: Box::new(right),
                operator
            };
        }

        left
    }

    fn parse_call_member_expr(&mut self) -> Statement {
        let member = self.parse_member_expr();

        if self.at() == Token::OpenParen {
            self.parse_call_expr(member)
        } else {
            member
        }
    }

    fn parse_call_expr(&mut self, caller: Statement) -> Statement {
        let mut call_expr = Statement::CallExpr {
            args: self.parse_args(),
            caller: Box::new(caller)
        };

        if self.at() == Token::OpenParen {
            call_expr = self.parse_call_expr(call_expr);
        }

        call_expr
    }

    fn parse_args(&mut self) -> Vec<Statement> {
        self.expect(Token::OpenParen, "Expected open parenthesis".to_string());

        let args = match self.at() {
            Token::CloseParen => vec![],
            _ => self.parse_arguments_list()
        };

        self.expect(Token::CloseParen, "Missing closing parenthesis on arguments".to_string());

        args
    }

    fn parse_arguments_list(&mut self) -> Vec<Statement> {
        let mut args = vec![self.parse_statement()];

        while self.at() == Token::Comma {
            self.eat();
            args.push(self.parse_statement());
        }

        args
    }

    fn parse_member_expr(&mut self) -> Statement {
        let mut object = self.parse_primary_expr();

        while self.at() == Token::Dot {
            self.eat();
            let property = self.parse_primary_expr();
            match property {
                Statement::Identifier(_) => (),
                statement => panic!("Invalid statement '{:?}'", statement)
            }

            object = Statement::MemberExpr {
                object: Box::new(object),
                property: Box::new(property)
            };
        }

        object
    }

    fn parse_primary_expr(&mut self) -> Statement {
        let token = self.eat();

        match token {
            Token::Object => self.parse_object(),
            Token::Identifier(value) => Statement::Identifier(value),
            Token::Number(number) => Statement::NumericLiteral(number.parse().expect("Failed to parse")),
            Token::UnaryOperator(operator) => self.parse_unary_expr(operator),
            Token::OpenParen => {
                let value = self.parse_expr();
                if let Token::CloseParen = self.eat() {
                    value
                } else {
                    panic!("Opened parentheses isn't closed!")
                }
            }
            _ => panic!("Unexpected token found during parsing: {:?}", token)
        }
    }

    fn parse_object(&mut self) -> Statement {
        self.expect(Token::OpenBracket, "The object has to be opened with an open bracket".to_string());

        let mut map = HashMap::new();

        while self.at() != Token::CloseBracket {
            let key = match self.eat() {
                Token::Identifier(name) => name,
                token => panic!("Invalid token '{:?}', should be identifier", token),
            };

            match self.at() {
                Token::Colon => {
                    self.eat();
                    let value = self.parse_statement();
                    map.insert(key.clone(), value);

                    if self.at() != Token::CloseBracket {
                        self.expect(Token::Comma, format!("Forgot to close '{}' with a comma", key));
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
                token => panic!("Invalid token '{:?}', should be ':', ',' or '}}'", token)
            }
        }

        self.expect(Token::CloseBracket, "The object wasn't closed with a close bracket".to_string());

        Statement::Object(map)
    }

    fn parse_unary_expr(&mut self, operator: String) -> Statement {
        let value = self.parse_expr();

        Statement::UnaryExpr { value: Box::new(value), operator }
    }

}
