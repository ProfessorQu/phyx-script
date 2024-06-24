use crate::frontend::ast::Statement;

use super::{environment::Environment, eval::{eval_assignment, eval_binary_expr, eval_call_expr, eval_comparison_expr, eval_for_loop, eval_function_declaration, eval_identifier, eval_if_statement, eval_member_expr, eval_object, eval_program, eval_shape, eval_unary_expr, eval_var_declaration}, values::RuntimeValue};


pub fn evaluate(ast_node: Statement, env: &mut Environment) -> Result<RuntimeValue, String> {
    match ast_node {
        Statement::Program { body } => eval_program(body, env),
        Statement::VarDeclaration { identifier, value } => eval_var_declaration(identifier, value.as_ref(), env),
        Statement::FunctionDeclaration { name, parameters, body } => eval_function_declaration(name, parameters, body, env),
        Statement::ForLoop { loop_var, range, body } => eval_for_loop(loop_var, &range, body, env),
        Statement::If { condition, body, else_body } => eval_if_statement(&condition, body, else_body, env),

        Statement::AssignmentExpr { assignee, value } => eval_assignment(&assignee, &value, env),
        Statement::NumericLiteral(value) => Ok(RuntimeValue::Number(value)),
        Statement::Identifier(symbol) => eval_identifier(symbol, env),

        Statement::BinaryExpr { left, right, operator } => eval_binary_expr(&left, &right, operator, env),
        Statement::UnaryExpr { value, operator } => eval_unary_expr(&value, operator, env),
        Statement::Comparison { left, right, operator } => eval_comparison_expr(&left, &right, operator, env),

        Statement::Shape(shape) => eval_shape(shape, env),
        Statement::Object(map) => eval_object(map, env),

        Statement::CallExpr { args, caller } => eval_call_expr(args, caller.as_ref(), env),
        Statement::MemberExpr { object, property } => eval_member_expr(&object, &property, env),
    }
}
