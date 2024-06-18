use crate::frontend::ast::Statement;

use super::{environment::Environment, eval::{eval_assignment, eval_binary_expr, eval_element, eval_identifier, eval_program, eval_shape, eval_var_declaration}, values::RuntimeValue};


pub fn evaluate(ast_node: Statement, env: &mut Environment) -> Result<RuntimeValue, String> {
    match ast_node {
        Statement::Program { body } => eval_program(body, env),
        Statement::VarDeclaration { identifier, value } => eval_var_declaration(identifier, value.as_ref().clone(), env),

        Statement::AssignmentExpr { assignee, value } => eval_assignment(
            assignee.as_ref().clone(),
            value.as_ref().clone(),
            env),
        Statement::NumericLiteral(value) => Ok(RuntimeValue::Number(value)),
        Statement::Identifier(symbol) => eval_identifier(symbol, env),
        Statement::BinaryExpr { left, right, operator } => eval_binary_expr(
            left.as_ref().clone(),
            right.as_ref().clone(),
            operator,
            env),

        Statement::Shape(shape) => eval_shape(shape, env),
        Statement::Element(map) => eval_element(map, env),
    }
}
