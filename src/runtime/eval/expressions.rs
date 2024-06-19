use std::borrow::Borrow;

use crate::{frontend::ast::Statement, runtime::{evaluate, Environment, RuntimeValue}};

pub fn eval_numeric_binary_expr(left: f32, right: f32, operator: String) -> Result<RuntimeValue, String> {
    match operator.as_str() {
        "+" => Ok(RuntimeValue::Number(left + right)),
        "-" => Ok(RuntimeValue::Number(left - right)),
        "*" => Ok(RuntimeValue::Number(left * right)),
        "/" => Ok(RuntimeValue::Number(left / right)),
        _ => Err(format!("Invalid operator: {:?}", operator))
    }
}

pub fn eval_binary_expr(left: &Statement, right: &Statement, operator: String, env: &mut Environment) -> Result<RuntimeValue, String> {
    let left_eval = evaluate(left.clone(), env)?;
    let right_eval = evaluate(right.clone(), env)?;

    if let RuntimeValue::Number(left_value) = left_eval {
        if let RuntimeValue::Number(right_value) = right_eval {
            return eval_numeric_binary_expr(left_value, right_value, operator);
        }
    }

    Err(format!("Invalid binary expression: '{:?} {} {:?}", left, operator, right))
}

pub fn eval_identifier(symbol: String, env: &mut Environment) -> Result<RuntimeValue, String> {
    env.lookup_var(symbol)
}

pub fn eval_assignment(assignee: &Statement, value: &Statement, env: &mut Environment) -> Result<RuntimeValue, String> {
    if let Statement::Identifier(name) = assignee {
        let value = evaluate(value.clone(), env)?;

        env.assign_var(name.clone(), value)
    } else if let Statement::MemberExpr { object, property } = assignee {
        let value = evaluate(value.clone(), env)?;

        let object_name = match object.borrow() {
            Statement::Identifier(name) => name,
            statement => return Err(format!("Invalid object statement: '{:?}'", statement))
        };

        let property_name = match property.borrow() {
            Statement::Identifier(name) => name,
            statement => return Err(format!("Invalid property statement: '{:?}'", statement))
        };

        let element_map = match env.lookup_var(object_name.clone())? {
            RuntimeValue::Element(map) => map,
            value => return Err(format!("Invalid element map: '{:?}'", value))
        };

        let mut map = element_map.clone();
        map.insert(property_name.clone(), value.clone());

        env.assign_var(object_name.clone(), RuntimeValue::Element(map))
    } else {
        Err(format!("Invalid lefthandside of assignment operation: {:?}", assignee))
    }
}

pub fn eval_call_expr(args: Vec<Statement>, caller: &Statement, env: &mut Environment) -> Result<RuntimeValue, String> {
    todo!()
}

pub fn eval_member_expr(object: &Statement, property: &Statement, env: &mut Environment) -> Result<RuntimeValue, String> {
    let object_name = match object {
        Statement::Identifier(name) => name,
        statement => return Err(format!("Invalid object statement: '{:?}'", statement))
    };

    let property_name = match property {
        Statement::Identifier(name) => name,
        statement => return Err(format!("Invalid property statement: '{:?}'", statement))
    };

    match env.lookup_var(object_name.clone())? {
        RuntimeValue::Element(map) => match map.get(property_name) {
            Some(value) => Ok(value.clone()),
            None => Err(format!("Object '{:?}' doesn't have property '{:?}'", object_name, property_name))
        },
        value => Err(format!("Invalid runtime value: '{:?}'", value))
    }
}
