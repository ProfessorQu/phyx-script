mod colors;
mod eval;
mod environment;
mod interpreter;
mod native_fns;
mod values;

pub use environment::Environment;
pub use interpreter::evaluate;
pub use values::{RuntimeValue, Function};
pub(crate) use eval::eval_object_update_expr;