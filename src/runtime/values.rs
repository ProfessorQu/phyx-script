#[derive(Debug, Clone)]
pub enum RuntimeValue {
    NumberValue(f64),
    BooleanValue(bool)
}
