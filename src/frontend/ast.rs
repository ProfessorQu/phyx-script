use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Clone)]
pub enum Statement {
    Program { body: Vec<Statement> },
    BinaryExpr { left: Box<Statement>, right: Box<Statement>, operator: String },
    Identifier(String),
    NumericLiteral(f32),
    Shape(String),
    Element(HashMap<String, Statement>)
}
