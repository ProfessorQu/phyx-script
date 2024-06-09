#![allow(dead_code)]

use std::{fs, io};

mod frontend;
use frontend::{tokenize, Parser};

mod runtime;
use runtime::{evaluate, Environment, RuntimeValue};

mod elements;

fn main() -> Result<(), String> {
    let code = fs::read_to_string("ball.phyx").expect("Failed to read file");
    // let stdin = io::stdin();
    // println!("{:?}", tokenize(code));
    let mut parser = Parser::new();
    let mut env = Environment::new(None);
    env.declare_var("true".to_string(), RuntimeValue::BooleanValue(true))?;
    env.declare_var("false".to_string(), RuntimeValue::BooleanValue(false))?;

    let ast = parser.produce_ast(code)?;
    println!("{ast:?}");
    println!("{:?}", evaluate(ast, &mut env));
    // let mut input = "".to_string();

    // loop {
    //     let _ = stdin.read_line(&mut input);
    //     let ast = parser.produce_ast(input.clone()).expect("Ok Program");
    //     println!("ast: {:#?}", ast);
    //     println!("result: {:?}", evaluate(ast, &mut environment));

    //     input = "".to_string();
    // }

    Ok(())
}
