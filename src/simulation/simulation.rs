use std::fs;

use crate::{frontend::Parser, runtime::{evaluate, Environment, RuntimeValue}};

use super::Element;

use nannou::prelude::*;

pub struct Simulation {
    elements: Vec<Element>
}

pub fn model(_app: &App) -> Simulation {
    let code = fs::read_to_string("ball.phyx").expect("Failed to read file");
    let mut parser = Parser::new();
    let mut env = Environment::new(None);
    env.declare_var("true".to_string(), RuntimeValue::Boolean(true)).expect("'true' already declared");
    env.declare_var("false".to_string(), RuntimeValue::Boolean(false)).expect("'false' already declared");

    let ast = parser.produce_ast(code).expect("Failed to generate ");

    if let RuntimeValue::Elements(elements) = evaluate(ast, &mut env).expect("Failed to evaluate") {
        return Simulation { elements }
    }

    panic!("The code doesn't return a list of elements!")
}

pub fn update(_app: &App, model: &mut Simulation, _update: Update) {
    for element in &mut model.elements {
        element.update();
    }
}

pub fn view(app: &App, model: &Simulation, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for element in &model.elements {
        element.draw(&draw);
    }

    draw.to_frame(app, &frame).expect("Failed to draw to frame");
}
