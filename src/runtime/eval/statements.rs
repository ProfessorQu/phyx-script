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
    let (start, stop, step) = match evaluate(range.clone(), env)? {
        RuntimeValue::Range(start, stop, step) => (start, stop, step),
        _ => return Err("Expected a range".to_string())
    };

    let mut result = RuntimeValue::Number(0.0);

    for i in (start..stop).step_by(step) {
        let mut scope = Environment::new(env.clone());
        scope.declare_var(loop_var.clone(), RuntimeValue::Number(i as f32))?;

        for statement in body.clone() {
            result = evaluate(statement, &mut scope)?;
        }

        let parent = scope.parent.expect("The scoped environment doesn't have a parent");
        for (varname, value) in parent.get_variables() {
            match env.lookup_var(varname.clone()) {
                Ok(_) => env.assign_var(varname, value)?,
                Err(_) => env.declare_var(varname, value)?
            };
        }
    }

    Ok(result)
}

pub fn eval_if_statement(condition: &Statement, body: Vec<Statement>, else_body: Vec<Statement>, env: &mut Environment) -> Result<RuntimeValue, String> {
    let boolean = match evaluate(condition.clone(), env)? {
        RuntimeValue::Boolean(boolean) => boolean,
        value => return Err(format!("Value '{}' is not a boolean", value))
    };

    let mut result = RuntimeValue::Number(0.0);
    let mut scope = Environment::new(env.clone());
    if boolean {
        for statement in body.clone() {
            result = evaluate(statement, &mut scope)?;
        }
    } else {
        for statement in else_body.clone() {
            result = evaluate(statement, &mut scope)?;
        }
    }

    let parent = scope.parent.expect("The scoped environment doesn't have a parent");
    for (varname, value) in parent.get_variables() {
        match env.lookup_var(varname.clone()) {
            Ok(_) => env.assign_var(varname, value)?,
            Err(_) => env.declare_var(varname, value)?
        };
    }

    Ok(result)
}
