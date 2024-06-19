use std::collections::HashMap;

use crate::{frontend::{ast::Statement, ShapeType}, runtime::{evaluate, Environment, RuntimeValue}};

pub fn eval_shape(shape: ShapeType, _env: &mut Environment) -> Result<RuntimeValue, String> {
    Ok(RuntimeValue::Shape(shape))
}

pub fn eval_element(map: HashMap<String, Statement>, env: &mut Environment) -> Result<RuntimeValue, String> {
    // let mut builder = ElementBuilder::new();

    // for (key, statement) in map {
    //     let value = evaluate(statement, env)?;

    //     builder = match (key.as_str(), value.clone()) {
    //         ("size", RuntimeValue::Number(number)) => builder.size(number),
    //         ("gravity", RuntimeValue::Number(number)) => builder.gravity(number),
    //         ("speed", RuntimeValue::Number(number)) => builder.speed(number),
    //         ("stroke", RuntimeValue::Number(number)) => builder.stroke(number),
    //         ("x", RuntimeValue::Number(number)) => builder.x(number),
    //         ("y", RuntimeValue::Number(number)) => builder.y(number),
    //         ("bounciness", RuntimeValue::Number(number)) => builder.bounciness(number),
    //         ("color", RuntimeValue::Color(color)) => builder.color(color),
    //         ("fixed", RuntimeValue::Boolean(boolean)) => builder.fixed(boolean),
    //         ("shape", RuntimeValue::Shape(shape)) => builder.shape(shape),
    //         _ => return Err(format!("Invalid key-value pair: {:?}: {:?}", key, value))
    //     }
    // }

    // Ok(RuntimeValue::Element(builder.build(&mut env.physics)))

    let mut var_map = HashMap::new();
    for (key, value) in map {
        var_map.insert(key, evaluate(value, env)?);
    }

    Ok(RuntimeValue::Element(var_map))
}