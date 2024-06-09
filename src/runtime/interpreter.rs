use crate::frontend::ast::Statement;

use super::{environment::Environment, values::RuntimeValue};

fn eval_program(body: Vec<Statement>, env: &mut Environment) -> Result<RuntimeValue, String> {
    let mut last_eval = RuntimeValue::NumberValue(0.0);

    for statement in body {
        last_eval = evaluate(statement, env)?;
    }

    Ok(last_eval)
}

fn eval_numeric_binary_expr(left: f64, right: f64, operator: String) -> Result<RuntimeValue, String> {
    match operator.as_str() {
        "+" => Ok(RuntimeValue::NumberValue(left + right)),
        "-" => Ok(RuntimeValue::NumberValue(left - right)),
        "*" => Ok(RuntimeValue::NumberValue(left * right)),
        "/" => Ok(RuntimeValue::NumberValue(left / right)),
        _ => Err(format!("Invalid operator: {:?}", operator))
    }
}

fn eval_binary_expr(left: Statement, right: Statement, operator: String, env: &mut Environment) -> Result<RuntimeValue, String> {
    let left_eval = evaluate(left.clone(), env)?;
    let right_eval = evaluate(right.clone(), env)?;

    if let RuntimeValue::NumberValue(left_value) = left_eval {
        if let RuntimeValue::NumberValue(right_value) = right_eval {
            return eval_numeric_binary_expr(left_value, right_value, operator);
        }
    }

    Err(format!("Invalid binary expression: '{:?} {} {:?}", left, operator, right))
}

fn eval_identifier(symbol: String, env: &mut Environment) -> Result<RuntimeValue, String> {
    env.lookup_var(symbol)
}

pub fn evaluate(ast_node: Statement, env: &mut Environment) -> Result<RuntimeValue, String> {
    match ast_node {
        Statement::NumericLiteral(value) => Ok(RuntimeValue::NumberValue(value)),
        Statement::BinaryExpr { left, right, operator } => eval_binary_expr(
            left.as_ref().clone(),
            right.as_ref().clone(),
            operator,
            env),
        Statement::Identifier(symbol) => eval_identifier(symbol, env),
        Statement::Program { body } => eval_program(body, env),
        Statement::Element(element) => todo!()
    }
}