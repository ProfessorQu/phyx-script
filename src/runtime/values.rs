use nannou::color::Rgb;

use crate::{frontend::ShapeType, simulation::Element};

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Elements(Vec<Element>),
    Element(Element),
    Shape(ShapeType),
    Color(Rgb<u8>),
    Number(f32),
    Boolean(bool)
}
