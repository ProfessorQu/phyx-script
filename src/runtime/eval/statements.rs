use std::collections::HashMap;

use crate::{frontend::ast::Statement, runtime::{evaluate, values::Function, Environment, RuntimeValue}};

pub fn eval_program(body: Vec<Statement>, env: &mut Environment) -> RuntimeValue {
    let mut last_eval = RuntimeValue::Number(0.0);

    for statement in body {
        last_eval = evaluate(statement, env);
    }

    last_eval
}

pub fn eval_var_declaration(identifier: String, value: &Statement, env: &mut Environment) -> RuntimeValue {
    let value = evaluate(value.clone(), env); 
    env.declare_var(identifier, value)
}

pub fn eval_function_declaration(name: String, parameters: Vec<String>, body: Vec<Statement>, env: &mut Environment) -> RuntimeValue {
    let func = RuntimeValue::Function(Function::new(name.clone(), parameters, body, env.clone()));

    env.declare_var(name, func)
}

pub fn eval_for_loop(loop_var: String, range: &Statement, body: Vec<Statement>, env: &mut Environment) -> RuntimeValue {
    let (start, stop, step) = match evaluate(range.clone(), env) {
        RuntimeValue::Range(start, stop, step) => (start, stop, step),
        _ => panic!("Expected a range")
    };

    let mut result = RuntimeValue::Number(0.0);

    for i in (start..stop).step_by(step) {
        let mut scope = Environment::new(env.clone(), false);
        scope.declare_var(loop_var.clone(), RuntimeValue::Number(i as f32));

        for statement in body.clone() {
            result = evaluate(statement, &mut scope);
        }

        let parent = scope.parent.expect("The scoped environment doesn't have a parent");
        env.merge(*parent);
    }

    result
}

pub fn eval_if_statement(condition: &Statement, body: Vec<Statement>, else_body: Vec<Statement>, env: &mut Environment) -> RuntimeValue {
    let boolean = match evaluate(condition.clone(), env) {
        RuntimeValue::Boolean(boolean) => boolean,
        value => panic!("Value '{}' is not a boolean", value)
    };

    let mut result = RuntimeValue::Number(0.0);
    let mut scope = Environment::new(env.clone(), false);
    if boolean {
        for statement in body.clone() {
            result = evaluate(statement, &mut scope);
        }
    } else {
        for statement in else_body.clone() {
            result = evaluate(statement, &mut scope);
        }
    }

    let parent = scope.parent.expect("The scoped environment doesn't have a parent");
    env.merge(*parent);

    result
}

pub fn eval_while_statement(condition: &Statement, body: Vec<Statement>, env: &mut Environment) -> RuntimeValue {
    let mut boolean = match evaluate(condition.clone(), env) {
        RuntimeValue::Boolean(boolean) => boolean,
        value => panic!("Value '{}' is not a boolean", value)
    };

    let mut result = RuntimeValue::Number(0.0);
    while boolean {
        let mut scope = Environment::new(env.clone(), false);

        for statement in body.clone() {
            result = evaluate(statement, &mut scope);
        }

        let parent = scope.parent.expect("The scoped environment doesn't have a parent");
        env.merge(*parent);

        boolean = match evaluate(condition.clone(), env) {
            RuntimeValue::Boolean(boolean) => boolean,
            value => panic!("Value '{}' is not a boolean", value)
        };
    }

    result
}

pub fn eval_object(map: HashMap<String, Statement>, env: &mut Environment) -> RuntimeValue {
    let mut var_map = HashMap::new();
    for (key, value) in map {
        var_map.insert(key, evaluate(value, env));
    }

    RuntimeValue::Object(var_map)
}
