use std::{collections::HashMap, fmt::Debug};

use super::ShapeType;

#[derive(Debug, Clone)]
pub enum Statement {
    Program { body: Vec<Statement> },
    VarDeclaration { identifier: String, value: Box<Statement> },
    FunctionDeclaration { name: String, parameters: Vec<String>, body: Vec<Statement> },
    ForLoop { loop_var: String, range: Box<Statement>, body: Vec<Statement> },

    AssignmentExpr { assignee: Box<Statement>, value: Box<Statement> },
    MemberExpr { object: Box<Statement>, property: Box<Statement> },
    CallExpr { args: Vec<Statement>, caller: Box<Statement> },

    BinaryExpr { left: Box<Statement>, right: Box<Statement>, operator: String },
    UnaryExpr { value: Box<Statement>, operator: String },
    Identifier(String),
    NumericLiteral(f32),

    Shape(ShapeType),
    Object(HashMap<String, Statement>)
}
