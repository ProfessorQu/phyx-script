use std::fs;

use crate::{frontend::Parser, runtime::{evaluate, Environment, RuntimeValue}};

use super::Element;

use nannou::prelude::*;

pub struct Model {
    elements: Vec<Element>
}

pub fn model(_app: &App) -> Model {
    let code = fs::read_to_string("ball.phyx").expect("Failed to read file");
    let mut parser = Parser::new();

    let mut env = Environment::new();

    let ast = parser.produce_ast(code).expect("Failed to generate ");

    if let RuntimeValue::Elements(elements) = evaluate(ast, &mut env).expect("Failed to evaluate") {
        return Model { elements }
    }

    panic!("The code doesn't return a list of elements!")
}

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    for element in &mut model.elements {
        element.update();
    }
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for element in &model.elements {
        element.draw(&draw);
    }

    draw.to_frame(app, &frame).expect("Failed to draw to frame");
}