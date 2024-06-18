use crate::{frontend::ast::Statement, runtime::{evaluate, Environment, RuntimeValue}};

pub fn eval_program(body: Vec<Statement>, env: &mut Environment) -> Result<RuntimeValue, String> {
    let mut elements = vec![];
    let mut last_eval = RuntimeValue::Number(0.0);

    for statement in body {
        let eval = evaluate(statement, env)?;
        if let RuntimeValue::Element(element) = eval.clone() {
            elements.push(element)
        }

        last_eval = eval;
    }

    if elements.is_empty() {
        Ok(last_eval)
    } else {
        Ok(RuntimeValue::Elements(elements))
    }
}

pub fn eval_var_declaration(identifier: String, value: Statement, env: &mut Environment) -> Result<RuntimeValue, String> {
    let value = evaluate(value, env)?; 
    env.declare_var(identifier, value)
}