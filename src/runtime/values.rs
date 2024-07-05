use std::{collections::HashMap, fmt::{Debug, Display}};

use nannou::color::Rgb;

use crate::frontend::{ast::Statement, ShapeType};

use super::Environment;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Vec<Statement>,
    pub declaration_env: Environment
}

impl Function {
    pub fn new(name: String, parameters: Vec<String>, body: Vec<Statement>, declaration_env: Environment) -> Self {
        Function {
            name,
            parameters,
            body,
            declaration_env
        }
    }
}

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Number(f32),
    Boolean(bool),

    NativeFn(fn(args: Vec<RuntimeValue>, env: &mut Environment) -> RuntimeValue),
    Function(Function),
    Range(i32, i32, usize),

    Object(HashMap<String, RuntimeValue>),
    Objects(Vec<RuntimeValue>),

    Shape(ShapeType),
    Color(Rgb<u8>),
    Note(String)
}

impl Display for RuntimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{}", number),
            Self::Boolean(boolean) => write!(f, "{}", boolean),

            Self::NativeFn(func) => write!(f, "native fn ({:?})", func),
            Self::Function(Function { name, parameters, body: _, declaration_env: _ }) => write!(f, "{}({:?})", name, parameters),
            Self::Range(start, stop, step) => write!(f, "range({}, {}, {})", start, stop, step),

            Self::Object(map) => write!(f, "Object {{ {:?} }}", map),
            Self::Objects(objects) => write!(f, "Objects {{ {:?} }}", objects),

            Self::Shape(shape) => write!(f, "{:?}", shape),
            Self::Color(color) => write!(f, "{:?}", color),
            Self::Note(note) => write!(f, "{}", note)
        }
    }
}
