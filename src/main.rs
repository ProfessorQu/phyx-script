#![allow(dead_code)]

use simulation::{update, model, view};

mod frontend;
mod runtime;
mod simulation;

fn main() -> Result<(), String> {
    nannou::app(model).update(update).simple_window(view).run();

    Ok(())

    // let mut parser = Parser::new();
    // let mut env = Environment::new(None);

    // let mut code = Default::default();
    // loop {
    //     let stdin = io::stdin();
    //     stdin.read_line(&mut code).expect("To succeed");

    //     let ast = parser.produce_ast(code.clone())?;

    //     println!("AST: {:?}", ast);
    //     println!("{:?}", evaluate(ast, &mut env));

    //     code.clear();
    // }
}
