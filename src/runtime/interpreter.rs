use std::collections::HashMap;

use crate::{simulation::ElementBuilder, frontend::ast::Statement};

use super::{environment::Environment, values::RuntimeValue};

fn eval_program(body: Vec<Statement>, env: &mut Environment) -> Result<RuntimeValue, String> {
    let mut elements = vec![];
    // let mut last_eval = RuntimeValue::Number(0.0);

    for statement in body {
        if let RuntimeValue::Element(element) = evaluate(statement, env)? {
            elements.push(element)
        }
    }

    Ok(RuntimeValue::Elements(elements))
}

fn eval_numeric_binary_expr(left: f32, right: f32, operator: String) -> Result<RuntimeValue, String> {
    match operator.as_str() {
        "+" => Ok(RuntimeValue::Number(left + right)),
        "-" => Ok(RuntimeValue::Number(left - right)),
        "*" => Ok(RuntimeValue::Number(left * right)),
        "/" => Ok(RuntimeValue::Number(left / right)),
        _ => Err(format!("Invalid operator: {:?}", operator))
    }
}

fn eval_binary_expr(left: Statement, right: Statement, operator: String, env: &mut Environment) -> Result<RuntimeValue, String> {
    let left_eval = evaluate(left.clone(), env)?;
    let right_eval = evaluate(right.clone(), env)?;

    if let RuntimeValue::Number(left_value) = left_eval {
        if let RuntimeValue::Number(right_value) = right_eval {
            return eval_numeric_binary_expr(left_value, right_value, operator);
        }
    }

    Err(format!("Invalid binary expression: '{:?} {} {:?}", left, operator, right))
}

fn eval_identifier(symbol: String, env: &mut Environment) -> Result<RuntimeValue, String> {
    env.lookup_var(symbol)
}

fn eval_shape(shape: String, _env: &mut Environment) -> Result<RuntimeValue, String> {
    Ok(RuntimeValue::Shape(shape.try_into()?))
}

fn eval_element(map: HashMap<String, Statement>, env: &mut Environment) -> Result<RuntimeValue, String> {
    let mut builder = ElementBuilder::new();

    for (key, statement) in map {
        let value = evaluate(statement, env)?;
        if let RuntimeValue::Number(number) = value {
            builder = match key.as_str() {
                "size" => builder.size(number),
                "gravity" => builder.gravity(number),
                "speed" => builder.speed(number),
                _ => return Err(format!("Invalid key '{:?} for element", key))
            };
        } else if let RuntimeValue::Shape(shape) = value {
            if key.as_str() == "shape" {
                builder = builder.shape(shape);
            }
        }
    }

    Ok(RuntimeValue::Element(builder.build()))
}

pub fn evaluate(ast_node: Statement, env: &mut Environment) -> Result<RuntimeValue, String> {
    match ast_node {
        Statement::NumericLiteral(value) => Ok(RuntimeValue::Number(value)),
        Statement::BinaryExpr { left, right, operator } => eval_binary_expr(
            left.as_ref().clone(),
            right.as_ref().clone(),
            operator,
            env),
        Statement::Identifier(symbol) => eval_identifier(symbol, env),
        Statement::Program { body } => eval_program(body, env),
        Statement::Shape(shape) => eval_shape(shape, env),
        Statement::Element(map) => eval_element(map, env)
    }
}
