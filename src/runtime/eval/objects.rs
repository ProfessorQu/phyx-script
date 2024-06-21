use std::collections::HashMap;

use crate::{frontend::{ast::Statement, ShapeType}, runtime::{evaluate, Environment, RuntimeValue}};

pub fn eval_shape(shape: ShapeType, _env: &mut Environment) -> Result<RuntimeValue, String> {
    Ok(RuntimeValue::Shape(shape))
}

pub fn eval_object(map: HashMap<String, Statement>, env: &mut Environment) -> Result<RuntimeValue, String> {
    let mut var_map = HashMap::new();
    for (key, value) in map {
        var_map.insert(key, evaluate(value, env)?);
    }

    Ok(RuntimeValue::Object(var_map))
}