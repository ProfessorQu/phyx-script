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

pub fn eval_unary_expr(value: &Statement, operator: String, env: &mut Environment) -> Result<RuntimeValue, String> {
    let value = evaluate(value.clone(), env)?;

    if let RuntimeValue::Number(number) = value {
        return match operator.as_str() {
            "-" => Ok(RuntimeValue::Number(-number)),
            op => Err(format!("Invalid unary operator: '{:?}'", op))
        };
    }

    Err(format!("Invalid value: '{:?}'", value))
}

pub fn eval_comparison_expr(left: &Statement, right: &Statement, operator: String, env: &mut Environment) -> Result<RuntimeValue, String> {
    let left_eval = evaluate(left.clone(), env)?;
    let right_eval = evaluate(right.clone(), env)?;

    let left_val = match left_eval {
        RuntimeValue::Number(number) => number,
        _ => return Err(format!("Invalid comparison expression: '{:?} {} {:?}", left, operator, right))
    };
    let right_val = match right_eval {
        RuntimeValue::Number(number) => number,
        _ => return Err(format!("Invalid comparison expression: '{:?} {} {:?}", left, operator, right))
    };

    match operator.as_str() {
        "==" => Ok(RuntimeValue::Boolean(left_val == right_val)),
        ">=" => Ok(RuntimeValue::Boolean(left_val >= right_val)),
        "<=" => Ok(RuntimeValue::Boolean(left_val <= right_val)),
        ">" => Ok(RuntimeValue::Boolean(left_val > right_val)),
        "<" => Ok(RuntimeValue::Boolean(left_val < right_val)),
        _ => Err(format!("Invalid operator: {:?}", operator))
    }
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

        let object_map = match env.lookup_var(object_name.clone())? {
            RuntimeValue::Object(map) => map,
            value => return Err(format!("Invalid object map: '{:?}'", value))
        };

        let mut map = object_map.clone();
        map.insert(property_name.clone(), value.clone());

        env.assign_var(object_name.clone(), RuntimeValue::Object(map))
    } else {
        Err(format!("Invalid lefthandside of assignment operation: {:?}", assignee))
    }
}

pub fn eval_call_expr(args: Vec<Statement>, caller: &Statement, env: &mut Environment) -> Result<RuntimeValue, String> {
    let mut values = vec![];
    for arg in args {
        values.push(evaluate(arg, env)?);
    }

    match evaluate(caller.clone(), env)? {
        RuntimeValue::NativeFn(func) => Ok(func(values, env)),
        RuntimeValue::Function { name, parameters, body } => {
            let declaration_env = env.resolve_mut(&name)?;
            let mut scope = Environment::new(declaration_env.clone());

            let num_params = parameters.len();
            if num_params != values.len() {
                return Err(format!("The function '{}' takes {} arguments but {} were given", name, num_params, values.len()))
            }

            for i in 0..num_params {
                let varname = parameters[i].clone();
                scope.declare_var(varname, values[i].clone())?;
            }

            let mut result = RuntimeValue::Number(0.0);

            for statement in body {
                result = evaluate(statement, &mut scope)?;
            }
            
            Ok(result)
        }
        runtimevalue => Err(format!("Cannot call value that is not a function: '{:?}'", runtimevalue))
    }
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
        RuntimeValue::Object(map) => match map.get(property_name) {
            Some(value) => Ok(value.clone()),
            None => Err(format!("Object '{:?}' doesn't have property '{:?}'", object_name, property_name))
        },
        value => Err(format!("Invalid runtime value: '{:?}'", value))
    }
}
