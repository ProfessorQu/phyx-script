use crate::simulation::ObjectBuilder;

use super::{Environment, RuntimeValue};

pub fn rgb(args: Vec<RuntimeValue>, _env: &mut Environment) -> RuntimeValue {
    match args.len() {
        len if len < 3 => panic!("Too few arguments passed into 'rgb'"),
        len if len > 3 => panic!("Too many arguments passed into 'rgb'"),
        _ => {
            let r = match &args[0] {
                RuntimeValue::Number(number) => *number as u8,
                value => panic!("Invalid argument to 'rgb': {:?}", value)
            };

            let g = match &args[1] {
                RuntimeValue::Number(number) => *number as u8,
                value => panic!("Invalid argument to 'rgb': {:?}", value)
            };

            let b = match &args[2] {
                RuntimeValue::Number(number) => *number as u8,
                value => panic!("Invalid argument to 'rgb': {:?}", value)
            };

            RuntimeValue::Color(nannou::color::rgb(r, g, b))
        },
    }
}

pub fn hsv(args: Vec<RuntimeValue>, _env: &mut Environment) -> RuntimeValue {
    match args.len() {
        len if len < 3 => panic!("Too few arguments passed into 'rgb'"),
        len if len > 3 => panic!("Too many arguments passed into 'rgb'"),
        _ => {
            let hue = match &args[0] {
                RuntimeValue::Number(number) => *number as f64,
                value => panic!("Invalid argument to 'rgb': {:?}", value)
            };

            let saturation = match &args[1] {
                RuntimeValue::Number(number) => *number as f64,
                value => panic!("Invalid argument to 'rgb': {:?}", value)
            };

            let value = match &args[2] {
                RuntimeValue::Number(number) => *number as f64,
                value => panic!("Invalid argument to 'rgb': {:?}", value)
            };

            let (r, g, b) = hsv::hsv_to_rgb(hue, saturation, value);

            RuntimeValue::Color(nannou::color::rgb(r, g, b))
        },
    }
}

pub fn add(args: Vec<RuntimeValue>, env: &mut Environment) -> RuntimeValue {
    let objects_ref = env.resolve_objects().expect("Failed to resolve objects");
    let mut objects = objects_ref.borrow_mut();
    let physics_ref = env.resolve_physics().expect("Failed to get physics");
    let mut physics = physics_ref.borrow_mut();

    for arg in &args {
        let map = match arg {
            RuntimeValue::Object(object) => object,
            _ => panic!("Argument '{:?}' is not an object", arg)
        };

        let mut builder = ObjectBuilder::new();
        for (key, value) in map {
            builder = match (key.as_str(), value.clone()) {
                ("size", RuntimeValue::Number(number)) => builder.size(number),
                ("gravity", RuntimeValue::Number(number)) => builder.gravity(number),
                ("speed", RuntimeValue::Number(number)) => builder.speed(number),
                ("stroke", RuntimeValue::Number(number)) => builder.stroke(number),
                ("x", RuntimeValue::Number(number)) => builder.x(number),
                ("y", RuntimeValue::Number(number)) => builder.y(number),
                ("bounciness", RuntimeValue::Number(number)) => builder.bounciness(number),
                ("color", RuntimeValue::Color(color)) => builder.color(color),
                ("fixed", RuntimeValue::Boolean(boolean)) => builder.fixed(boolean),
                ("shape", RuntimeValue::Shape(shape)) => builder.shape(shape),
                _ => panic!("Invalid key-value pair: {:?}: {:?}", key, value)
            }
        }

        let object = builder.build(&mut physics);
        objects.push(object);
    }

    RuntimeValue::Number(0.0)
}

pub fn range(args: Vec<RuntimeValue>, _env: &mut Environment) -> RuntimeValue {
    match args.len() {
        len if len != 1 => panic!("Invalid number of arguments to range takes 1, given {}", len),
        _ => {
            if let RuntimeValue::Number(number) = args[0] {
                RuntimeValue::Range(number as usize)
            } else {
                panic!("Argument: {:?} is not a number", args[0])
            }
        }
    }
}
