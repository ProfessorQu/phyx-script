mod values;
mod interpreter;
mod environment;

pub use values::RuntimeValue;
pub use interpreter::evaluate;
pub use environment::Environment;