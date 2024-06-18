mod values;
mod interpreter;
mod environment;
mod eval;

pub use values::RuntimeValue;
pub use interpreter::evaluate;
pub use environment::Environment;