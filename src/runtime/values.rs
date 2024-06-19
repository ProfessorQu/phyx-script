use std::{collections::HashMap, fmt::Debug};

use nannou::color::Rgb;

use crate::frontend::ShapeType;

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Number(f32),
    Boolean(bool),

    Element(HashMap<String, RuntimeValue>),

    Shape(ShapeType),
    Color(Rgb<u8>),
}
