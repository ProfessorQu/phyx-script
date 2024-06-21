use std::{collections::HashMap, fmt::Debug};

use nannou::color::Rgb;

use crate::frontend::{ast::Statement, ShapeType};

use super::Environment;

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Number(f32),
    Boolean(bool),

    NativeFn(fn(args: Vec<RuntimeValue>, env: &mut Environment) -> RuntimeValue),
    Function{ name: String, parameters: Vec<String>, body: Vec<Statement>, declaration_env: Box<Environment> },
    Range(usize),

    Object(HashMap<String, RuntimeValue>),
    Objects(Vec<RuntimeValue>),

    Shape(ShapeType),
    Color(Rgb<u8>),
}
