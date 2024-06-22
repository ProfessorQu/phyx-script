use std::{cmp::Ordering, env, fs};

use crate::{frontend::Parser, runtime::{evaluate, Environment, RuntimeValue}, simulation::ObjectBuilder};

use super::{physics::Physics, Object};

use nannou::prelude::*;

pub struct Model {
    physics: Physics,
    objects: Vec<Object>
}

pub fn model(_app: &App) -> Model {
    let args: Vec<String> = env::args().collect();
    
    match args.len().cmp(&2) {
        Ordering::Less => panic!("Please input a file to run"),
        Ordering::Greater => panic!("Too many arguments"),
        Ordering::Equal => ()
    }

    let filename = &args[1];
    let code = fs::read_to_string(filename).expect("Failed to read file");
    let mut parser = Parser::new();
    let mut env = Environment::new_global();

    let ast = parser.produce_ast(code).expect("Failed to generate ");
    evaluate(ast, &mut env).expect("Failed to evaluate");

    let mut physics = Physics::new();
    let values = match env.lookup_var("objects".to_string()).expect("Failed to get objects") {
        RuntimeValue::Objects(objects) => objects,
        _ => panic!("Invalid 'objects'")
    };

    let mut objects = vec![];
    add_objects(&values, &mut objects, &mut physics);

    Model { objects, physics }
}

fn add_objects(values: &Vec<RuntimeValue>, objects: &mut Vec<Object>, physics: &mut Physics) {
    for value in values {
        if let RuntimeValue::Object(object_map) = value {
            objects.push(ObjectBuilder::from_map(object_map.clone(), physics));
        } else if let RuntimeValue::Objects(values) = value {
            add_objects(values, objects, physics);
        } else {
            panic!("Not an object: {:?}", value);
        }
    }
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
