use std::{collections::HashMap, fmt::Debug};

use super::{lexer::VarType, ShapeType};

#[derive(Debug, Clone)]
pub enum Statement {
    Program { body: Vec<Statement> },
    BinaryExpr { left: Box<Statement>, right: Box<Statement>, operator: String },
    Identifier(String),
    NumericLiteral(f32),
    Shape(ShapeType),
    Element(HashMap<VarType, Statement>)
}
