use std::borrow::Borrow;

use crate::{frontend::ast::Statement, runtime::{evaluate, Environment, RuntimeValue}};

pub fn eval_numeric_binary_expr(left: f32, right: f32, operator: String) -> Result<RuntimeValue, String> {
    match operator.as_str() {
        "+" => Ok(RuntimeValue::Number(left + right)),
        "-" => Ok(RuntimeValue::Number(left - right)),
        "*" => Ok(RuntimeValue::Number(left * right)),
        "/" => Ok(RuntimeValue::Number(left / right)),
        "%" => Ok(RuntimeValue::Number(left % right)),
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

    match value {
        RuntimeValue::Number(number) => {
            match operator.as_str() {
                "-" => Ok(RuntimeValue::Number(-number)),
                op => Err(format!("Invalid unary operator: '{:?}'", op))
            }
        }
        RuntimeValue::Boolean(boolean) => {
            match operator.as_str() {
                "!" => Ok(RuntimeValue::Boolean(!boolean)),
                op => Err(format!("Invalid unary operator: '{:?}'", op))
            }
        }
        _ => Err(format!("Invalid value: '{:?}'", value))
    }

}

pub fn eval_comparison_expr(left: &Statement, right: &Statement, operator: String, env: &mut Environment) -> Result<RuntimeValue, String> {
    let left_eval = evaluate(left.clone(), env)?;
    let right_eval = evaluate(right.clone(), env)?;

    match (left_eval.clone(), right_eval.clone()) {
        (RuntimeValue::Number(left_val), RuntimeValue::Number(right_val)) => eval_numeric_comparison_expr(left_val, right_val, operator),
        (RuntimeValue::Boolean(left_val), RuntimeValue::Boolean(right_val)) => eval_other_comparison_expr(left_val, right_val, operator),
        (RuntimeValue::Color(left_val), RuntimeValue::Color(right_val)) => eval_other_comparison_expr(left_val, right_val, operator),
        _ => Err(format!("Invalid comparison: {:?} to {:?}", left_eval, right_eval))
    }
}

fn eval_numeric_comparison_expr(left_val: f32, right_val: f32, operator: String) -> Result<RuntimeValue, String> {
    match operator.as_str() {
        "==" => Ok(RuntimeValue::Boolean(left_val == right_val)),
        "!=" => Ok(RuntimeValue::Boolean(left_val != right_val)),
        ">=" => Ok(RuntimeValue::Boolean(left_val >= right_val)),
        "<=" => Ok(RuntimeValue::Boolean(left_val <= right_val)),
        ">" => Ok(RuntimeValue::Boolean(left_val > right_val)),
        "<" => Ok(RuntimeValue::Boolean(left_val < right_val)),
        _ => Err(format!("Invalid operator: {:?}", operator))
    }
}

fn eval_other_comparison_expr<T>(left_val: T, right_val: T, operator: String) -> Result<RuntimeValue, String> 
    where T: PartialEq {
    match operator.as_str() {
        "==" => Ok(RuntimeValue::Boolean(left_val == right_val)),
        "!=" => Ok(RuntimeValue::Boolean(left_val != right_val)),
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
    for arg in args.clone() {
        values.push(evaluate(arg, env)?);
    }

    match evaluate(caller.clone(), env)? {
        RuntimeValue::NativeFn(func) => Ok(func(values, env)),
        RuntimeValue::Function { name, parameters, body, declaration_env } => {
            let mut scope = Environment::new(declaration_env);

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

            let mut top_env = scope.clone();
            while let Some(env) = top_env.parent {
                top_env = *env;
            }

            env.merge_objects(top_env);


            Ok(result)
        }
        RuntimeValue::Objects(mut objects) => {
            objects.extend(values);
            env.assign_var("objects".to_string(), RuntimeValue::Objects(objects))
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
        }
        RuntimeValue::Objects(objects) => Ok(RuntimeValue::Objects(objects)),
        value => Err(format!("Invalid runtime value: '{:?}'", value))
    }
}
