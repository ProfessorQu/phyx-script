use nannou::color::Rgb;

use crate::{frontend::ShapeType, simulation::Element};

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Number(f32),
    Boolean(bool),

    Element(Element),
    Elements(Vec<Element>),

    Shape(ShapeType),
    Color(Rgb<u8>),
}
