use crate::{frontend::ast::Statement, runtime::{evaluate, Environment, RuntimeValue}};

pub fn eval_program(body: Vec<Statement>, env: &mut Environment) -> Result<RuntimeValue, String> {
    let mut last_eval = RuntimeValue::Number(0.0);

    for statement in body {
        last_eval = evaluate(statement, env)?;
    }

    Ok(last_eval)
}

pub fn eval_var_declaration(identifier: String, value: &Statement, env: &mut Environment) -> Result<RuntimeValue, String> {
    let value = evaluate(value.clone(), env)?; 
    env.declare_var(identifier, value)
}