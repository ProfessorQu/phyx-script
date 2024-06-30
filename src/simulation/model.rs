use std::{cmp::Ordering, env, fs};

use crate::{frontend::Parser, runtime::{evaluate, Environment, RuntimeValue}, simulation::ObjectBuilder};

use super::{physics::Physics, Object};

use nannou::{prelude::*, winit::window::Icon};

pub struct Model {
    physics: Physics,
    objects: Vec<Object>,
    background_color: Rgb<u8>
}

pub fn model(app: &App) -> Model {
    let args: Vec<String> = env::args().collect();
    
    match args.len().cmp(&2) {
        Ordering::Less => panic!("Please input a file to run"),
        Ordering::Greater => panic!("Too many arguments"),
        Ordering::Equal => ()
    }

    app.main_window().set_maximized(true);

    let decoder = png::Decoder::new(fs::File::open("assets/icon.png").unwrap());
    let mut reader = decoder.read_info().expect("Failed to read info of icon");
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).expect("Failed to read the next frame");
    let bytes = &buf[..info.buffer_size()].to_vec();
    let icon = Icon::from_rgba(bytes.clone(), 180, 180).expect("Failed to create icon");
    app.main_window().set_window_icon(Some(icon));

    let filename = &args[1];
    let title = "Phyx - ".to_string() + filename.split('/').last().expect("Filename is empty");
    app.main_window().set_title(title.as_str());

    let code = fs::read_to_string(filename).expect("Failed to read file");
    let mut parser = Parser::new();
    let mut global_env = Environment::new_global();

    let ast = parser.produce_ast(code);
    evaluate(ast, &mut global_env);

    let mut physics = Physics::new();
    let values = match global_env.lookup_var("objects".to_string()) {
        RuntimeValue::Objects(objects) => objects,
        _ => panic!("Invalid 'objects'")
    };

    let mut objects = vec![];
    add_objects(&values, &mut objects, &mut physics);

    let background_color = match global_env.lookup_var("background_color".to_string()) {
        RuntimeValue::Color(color) => color,
        value => panic!("Invalid value for background: {:?}", value)
    };

    Model { objects, physics, background_color }
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
    for object in &mut model.objects {
        object.update(&mut model.physics);
    }

    model.physics.step();
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(model.background_color);

    for object in &model.objects {
        object.draw(&draw, &model.physics);
    }

    draw.to_frame(app, &frame).expect("Failed to draw to frame");
}
