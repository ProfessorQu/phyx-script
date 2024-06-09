use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Clone)]
pub enum Statement {
    Program { body: Vec<Statement> },
    BinaryExpr { left: Box<Statement>, right: Box<Statement>, operator: String },
    Identifier(String),
    NumericLiteral(f64),
    Element(HashMap<String, Statement>)
}
