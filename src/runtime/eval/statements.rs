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

pub fn eval_function_declaration(name: String, parameters: Vec<String>, body: Vec<Statement>, env: &mut Environment) -> Result<RuntimeValue, String> {
    let func = RuntimeValue::Function { name: name.clone(), parameters, body, declaration_env: env.clone() };

    env.declare_var(name, func)
}

pub fn eval_for_loop(loop_var: String, range: &Statement, body: Vec<Statement>, env: &mut Environment) -> Result<RuntimeValue, String> {
    let stop = match evaluate(range.clone(), env)? {
        RuntimeValue::Range(stop) => stop,
        _ => return Err("Expected a range".to_string())
    };

    let mut scope = Environment::new(env.clone());
    scope.declare_var(loop_var.clone(), RuntimeValue::Number(0.0))?;

    let mut result = RuntimeValue::Number(0.0);

    for i in 0..stop {
        scope.assign_var(loop_var.clone(), RuntimeValue::Number(i as f32))?;

        for statement in body.clone() {
            result = evaluate(statement, &mut scope)?;
        }
    }

    Ok(result)
}
