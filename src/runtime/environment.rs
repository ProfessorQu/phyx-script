use std::{collections::HashMap, fmt::Debug};

use nannou::color::*;

use crate::{frontend::ShapeType, runtime::values::RuntimeValue};

use super::native_fns;

#[derive(Debug, Clone)]
pub struct Environment {
    pub parent: Option<Box<Self>>,
    variables: HashMap<String, RuntimeValue>
}

impl Environment {
    pub fn new(parent: Self) -> Self {
        Self {
            parent: Some(Box::new(parent)),
            variables: HashMap::new()
        }
    }

    pub fn new_global() -> Self {
        let mut env = Self {
            parent: None,
            variables: HashMap::new()
        };

        for (name, color) in &super::colors::COLORS {
            env.declare_var(name.to_string(), RuntimeValue::Color(Rgb::new(
                color.red, color.green, color.green
            ))).unwrap_or_else(|_| panic!("'{:?}' already declared", name));
        }

        env.declare_var("objects".to_string(), RuntimeValue::Objects(vec![])).expect("'object' already declared");
        env.declare_var("background_color".to_string(), RuntimeValue::Color(BLACK)).expect("'background_color' already declared");

        env.declare_var("true".to_string(), RuntimeValue::Boolean(true)).expect("'true' already declared");
        env.declare_var("false".to_string(), RuntimeValue::Boolean(false)).expect("'false' already declared");

        env.declare_var("circle".to_string(), RuntimeValue::Shape(ShapeType::Circle)).expect("'circle' already declared");
        env.declare_var("rect".to_string(), RuntimeValue::Shape(ShapeType::Rect)).expect("'rect' already declared");
        env.declare_var("ring".to_string(), RuntimeValue::Shape(ShapeType::Ring)).expect("'ring' already declared");

        env.declare_var("print".to_string(), RuntimeValue::NativeFn(native_fns::print)).expect("'print' already declared");
        env.declare_var("rgb".to_string(), RuntimeValue::NativeFn(native_fns::rgb)).expect("'rgb' already declared");
        env.declare_var("hsv".to_string(), RuntimeValue::NativeFn(native_fns::hsv)).expect("'rgb' already declared");

        env.declare_var("range".to_string(), RuntimeValue::NativeFn(native_fns::range)).expect("'range' already declared");
        env.declare_var("random".to_string(), RuntimeValue::NativeFn(native_fns::random)).expect("'random' already declared");
        env.declare_var("floor".to_string(), RuntimeValue::NativeFn(native_fns::floor)).expect("'floor' already declared");
        env.declare_var("ceil".to_string(), RuntimeValue::NativeFn(native_fns::ceil)).expect("'ceil' already declared");

        env
    }

    pub fn declare_var(&mut self, varname: String, value: RuntimeValue) -> Result<RuntimeValue, String> {
        if self.variables.contains_key(&varname) {
            return Err(format!("Cannot declare variable '{:?}' as it's already defined", varname))
        }

        self.variables.insert(varname, value.clone());
        Ok(value)
    }

    pub fn assign_var(&mut self, varname: String, value: RuntimeValue) -> Result<RuntimeValue, String> {
        let env = self.resolve_mut(&varname)?;
        env.variables.insert(varname, value.clone());

        Ok(value)
    }

    pub fn lookup_var(&self, varname: String) -> Result<RuntimeValue, String> {
        let env = self.resolve(&varname)?;

        Ok(env.variables.get(&varname).expect("'resolve' succeeded but varname is not present").clone())
    }

    pub fn resolve(&self, varname: &String) -> Result<&Environment, String> {
        if self.variables.contains_key(varname) {
            Ok(self)
        } else if let Some(parent) = &self.parent {
            parent.resolve(varname)
        } else {
            Err(format!("Failed to resolve variable '{}'", varname))
        }
    }

    pub fn resolve_mut(&mut self, varname: &String) -> Result<&mut Environment, String> {
        if self.variables.contains_key(varname) {
            Ok(self)
        } else if let Some(parent) = &mut self.parent {
            parent.resolve_mut(varname)
        } else {
            Err(format!("Failed to resolve mutable variable '{:?}'", varname))
        }
    }

    pub fn get_variables(&self) -> HashMap<String, RuntimeValue> {
        self.variables.clone()
    }
}
