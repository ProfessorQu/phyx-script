#![allow(dead_code)]

use std::io;

use frontend::Parser;
use runtime::{evaluate, Environment};
use simulation::{update, model, view};

mod frontend;
mod runtime;
mod simulation;

fn main() -> Result<(), String> {
    // nannou::app(model).update(update).simple_window(view).run();

    let mut parser = Parser::new();
    let mut env = Environment::new(None);
    env.declare_var("true".to_string(), runtime::RuntimeValue::Boolean(true)).expect("'true' already declared");
    env.declare_var("false".to_string(), runtime::RuntimeValue::Boolean(false)).expect("'false' already declared");

    let mut code = Default::default();
    loop {
        let stdin = io::stdin();
        stdin.read_line(&mut code).expect("To succeed");

        let ast = parser.produce_ast(code.clone())?;

        println!("AST: {:?}", ast);
        println!("{:?}", evaluate(ast, &mut env));

        code.clear();
    }
}
