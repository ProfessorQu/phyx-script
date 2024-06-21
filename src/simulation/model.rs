use std::fs;

use crate::{frontend::Parser, runtime::{evaluate, Environment}};

use super::{physics::Physics, Object};

use nannou::prelude::*;

pub struct Model {
    physics: Physics,
    objects: Vec<Object>
}

pub fn model(_app: &App) -> Model {
    let code = fs::read_to_string("ball.phyx").expect("Failed to read file");
    let mut parser = Parser::new();

    let mut env = Environment::new_global();

    let ast = parser.produce_ast(code).expect("Failed to generate ");

    println!("AST: {:?}", ast);
    println!("result: {:?}", evaluate(ast, &mut env).expect("Failed to evaluate"));

    let objects = match env.objects {
        Some(objects) => objects.borrow().to_vec(),
        None => panic!("No objects")
    };
    let physics = match env.physics {
        Some(physics) => physics.as_ref().clone(),
        None => panic!("No physics")
    };

    Model { objects, physics }
}

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    model.physics.step();
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for object in &model.objects {
        object.draw(&draw, &model.physics);
    }

    draw.to_frame(app, &frame).expect("Failed to draw to frame");
}
