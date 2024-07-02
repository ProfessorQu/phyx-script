use std::{collections::HashMap, fmt::Debug};

use nannou::color::*;

use crate::{frontend::ShapeType, runtime::values::RuntimeValue};

use super::native_fns;

#[derive(Debug, Clone)]
pub struct Environment {
    pub parent: Option<Box<Self>>,
    variables: HashMap<String, RuntimeValue>,
    func_env: bool
}

impl Environment {
    pub fn new(parent: Self, func_env: bool) -> Self {
        Self {
            parent: Some(Box::new(parent)),
            variables: HashMap::new(),
            func_env
        }
    }

    pub fn new_global() -> Self {
        let mut env = Self {
            parent: None,
            variables: HashMap::new(),
            func_env: false
        };

        for (name, color) in &super::colors::COLORS {
            env.declare_var(name.to_string(), RuntimeValue::Color(Rgb::new(
                color.red, color.green, color.green
            )));
        }

        env.declare_var("objects".to_string(), RuntimeValue::Objects(vec![]));
        env.declare_var("background_color".to_string(), RuntimeValue::Color(BLACK));

        env.declare_var("true".to_string(), RuntimeValue::Boolean(true));
        env.declare_var("false".to_string(), RuntimeValue::Boolean(false));

        env.declare_var("circle".to_string(), RuntimeValue::Shape(ShapeType::Circle));
        env.declare_var("rect".to_string(), RuntimeValue::Shape(ShapeType::Rect));
        env.declare_var("ring".to_string(), RuntimeValue::Shape(ShapeType::Ring));

        env.declare_var("print".to_string(), RuntimeValue::NativeFn(native_fns::print));
        env.declare_var("rgb".to_string(), RuntimeValue::NativeFn(native_fns::rgb));
        env.declare_var("hsv".to_string(), RuntimeValue::NativeFn(native_fns::hsv));

        env.declare_var("range".to_string(), RuntimeValue::NativeFn(native_fns::range));
        env.declare_var("random".to_string(), RuntimeValue::NativeFn(native_fns::random));
        env.declare_var("floor".to_string(), RuntimeValue::NativeFn(native_fns::floor));
        env.declare_var("ceil".to_string(), RuntimeValue::NativeFn(native_fns::ceil));

        env.declare_var("abs".to_string(), RuntimeValue::NativeFn(native_fns::abs));
        env.declare_var("pow".to_string(), RuntimeValue::NativeFn(native_fns::pow));
        env.declare_var("sqrt".to_string(), RuntimeValue::NativeFn(native_fns::sqrt));

        env
    }

    pub fn declare_var(&mut self, varname: String, value: RuntimeValue) -> RuntimeValue {
        if self.variables.contains_key(&varname) {
            panic!("Cannot declare variable '{:?}' as it's already defined", varname)
        }

        self.variables.insert(varname, value.clone());
        value
    }

    pub fn assign_var(&mut self, varname: String, value: RuntimeValue) -> RuntimeValue {
        let env = self.resolve_mut(&varname);
        env.variables.insert(varname, value.clone());

        value
    }

    pub fn lookup_var(&self, varname: String) -> RuntimeValue {
        let env = self.resolve(&varname);

        env.variables.get(&varname).expect("'resolve' succeeded but varname is not present").clone()
    }

    pub fn resolve(&self, varname: &String) -> &Environment {
        if self.variables.contains_key(varname) {
            self
        } else if let Some(parent) = &self.parent {
            parent.resolve(varname)
        } else {
            panic!("Failed to resolve variable '{}'", varname)
        }
    }

    pub fn resolve_mut(&mut self, varname: &String) -> &mut Environment {
        if self.variables.contains_key(varname) {
            self
        } else if self.func_env {
            panic!("Can't resolve mutable variable '{}' because this is a function environment", varname)
        } else if let Some(parent) = &mut self.parent {
            parent.resolve_mut(varname)
        } else {
            panic!("Failed to resolve mutable variable '{:?}'", varname)
        }
    }

    pub fn get_variables(&self) -> HashMap<String, RuntimeValue> {
        self.variables.clone()
    }

    pub fn merge(&mut self, other: Environment) {
        for (varname, value) in other.get_variables() {
            self.assign_var(varname, value);
        }

        if other.func_env {
            return
        }

        if let Some(parent) = other.parent {
            self.merge(*parent);
        }
    }

    pub fn merge_objects(&mut self, other: Environment) {
        for (varname, value) in other.get_variables() {
            match self.lookup_var(varname.clone()) {
                RuntimeValue::Objects(mut objects) => {
                    if let RuntimeValue::Objects(scope_objects) = other.lookup_var("objects".to_string()) {
                        objects.extend(scope_objects);

                        self.assign_var(varname, RuntimeValue::Objects(objects));
                    }
                }
                _ => {
                    self.assign_var(varname, value);
                }
            };
        }

        if other.func_env {
            return
        }

        if let Some(parent) = other.parent {
            self.merge(*parent);
        }
    }
}
