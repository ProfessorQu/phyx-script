use std::{collections::HashMap, fmt::Debug};

use nannou::color::Rgb;

use crate::frontend::ShapeType;

use super::Environment;

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Number(f32),
    Boolean(bool),

    NativeFn(fn(args: Vec<RuntimeValue>, env: &mut Environment) -> RuntimeValue),

    Object(HashMap<String, RuntimeValue>),

    Shape(ShapeType),
    Color(Rgb<u8>),
}
