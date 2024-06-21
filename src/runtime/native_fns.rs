use nannou::color;
use rand::Rng;
use std::fmt::Write;

use super::{Environment, RuntimeValue};

pub fn print(args: Vec<RuntimeValue>, _env: &mut Environment) -> RuntimeValue {
    let joined: String = args.iter().fold(String::new(), |mut output, arg| {
        let _ = write!(output, "{} ", arg);
        output
    });

    println!("{}", joined);
    RuntimeValue::Number(0.0)
}

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
            RuntimeValue::Color(color::rgb(r, g, b))
        },
    }
}

pub fn range(args: Vec<RuntimeValue>, _env: &mut Environment) -> RuntimeValue {
    match args.len() {
        1 => {
            let stop = match &args[0] {
                RuntimeValue::Number(number) => *number as i32,
                arg => panic!("Argument: {:?} is not a number", arg)
            };

            RuntimeValue::Range(0, stop, 1)
        }
        2 => {
            let start = match &args[0] {
                RuntimeValue::Number(number) => *number as i32,
                arg => panic!("Argument: {:?} is not a number", arg)
            };

            let stop = match &args[1] {
                RuntimeValue::Number(number) => *number as i32,
                arg => panic!("Argument: {:?} is not a number", arg)
            };

            RuntimeValue::Range(start, stop, 1)
        }
        3 => {
            let start = match &args[0] {
                RuntimeValue::Number(number) => *number as i32,
                arg => panic!("Argument: {:?} is not a number", arg)
            };

            let stop = match &args[1] {
                RuntimeValue::Number(number) => *number as i32,
                arg => panic!("Argument: {:?} is not a number", arg)
            };

            let step = match &args[2] {
                RuntimeValue::Number(number) => *number as usize,
                arg => panic!("Argument: {:?} is not a number", arg)
            };

            RuntimeValue::Range(start, stop, step)
        }
        len => panic!("Invalid number of arguments to range takes 1, given {}", len),
    }
}

pub fn random(args: Vec<RuntimeValue>, _env: &mut Environment) -> RuntimeValue {
    match args.len() {
        0 => {
            RuntimeValue::Number(rand::thread_rng().gen())
        }
        1 => {
            let stop = match args[0] {
                RuntimeValue::Number(number) => number,
                _ => panic!("Invalid argument to 'random'")
            };

            RuntimeValue::Number(rand::thread_rng().gen_range(0.0..stop))
        }
        2 => {
            let start = match args[0] {
                RuntimeValue::Number(number) => number,
                _ => panic!("Invalid argument to 'random'")
            };

            let stop = match args[1] {
                RuntimeValue::Number(number) => number,
                _ => panic!("Invalid argument to 'random'")
            };

            RuntimeValue::Number(rand::thread_rng().gen_range(start..stop))
        }
        _ => panic!("Invalid number of arguments to 'random'")
    }
}

pub fn floor(args: Vec<RuntimeValue>, _env: &mut Environment) -> RuntimeValue {
    if args.len() != 1 {
        panic!("Invalid number of arguments to 'floor' function")
    }

    if let RuntimeValue::Number(number) = args[0] {
        RuntimeValue::Number(number.floor())
    } else {
        panic!("Invalid input to 'floor' expected number")
    }
}

pub fn ceil(args: Vec<RuntimeValue>, _env: &mut Environment) -> RuntimeValue {
    if args.len() != 1 {
        panic!("Invalid number of arguments to 'ceil' function")
    }

    if let RuntimeValue::Number(number) = args[0] {
        RuntimeValue::Number(number.ceil())
    } else {
        panic!("Invalid input to 'ceil' expected number")
    }
}
