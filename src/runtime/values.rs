use crate::simulation::{Element, ShapeType};

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Elements(Vec<Element>),
    Element(Element),
    Shape(ShapeType),
    Number(f32),
    Boolean(bool)
}
