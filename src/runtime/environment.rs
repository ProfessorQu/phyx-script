use std::collections::HashMap;

use nannou::color::*;
use palette::named::from_str;

use crate::{runtime::values::RuntimeValue, simulation::{Object, Physics}};

use super::native_fns;

pub struct Environment {
    parent: Box<Option<Self>>,
    pub physics: Option<Physics>,
    pub objects: Option<Vec<Object>>,
    variables: HashMap<String, RuntimeValue>
}

impl Environment {
    pub fn new(parent: Self) -> Self {
        Self {
            parent: Box::new(Some(parent)),
            physics: None,
            objects: None,
            variables: HashMap::new()
        }
    }

    pub fn new_global() -> Self {
        let physics = Some(Physics::new());
        let objects = Some(vec![]);

        let mut env = Self {
            parent: Box::new(None),
            physics,
            objects,
            variables: HashMap::new()
        };

        env.declare_var("true".to_string(), RuntimeValue::Boolean(true)).expect("'true' already declared");
        env.declare_var("false".to_string(), RuntimeValue::Boolean(false)).expect("'false' already declared");

        env.declare_var("print".to_string(), RuntimeValue::NativeFn(|args, _env| {
            println!("{:?}", args);
            RuntimeValue::Number(0.0)
        })).expect("'print' already declared");

        env.declare_var("rgb".to_string(), RuntimeValue::NativeFn(native_fns::rgb)).expect("'rgb' already declared");
        env.declare_var("hsv".to_string(), RuntimeValue::NativeFn(native_fns::hsv)).expect("'rgb' already declared");
        env.declare_var("add".to_string(), RuntimeValue::NativeFn(native_fns::add)).expect("'add' already declared");

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
        if let Some(color) = from_str(varname.as_str()) {
            return Ok(RuntimeValue::Color(Rgb::new(
                color.red,
                color.green,
                color.blue
            )))
        }

        let env = self.resolve(&varname)?;

        Ok(env.variables.get(&varname).expect("'resolve' succeeded but varname is not present").clone())
    }

    pub fn resolve(&self, varname: &String) -> Result<&Environment, String> {
        if self.variables.contains_key(varname) {
            Ok(self)
        } else if let Some(box_parent) = self.parent.as_ref() {
            box_parent.resolve(varname)
        } else {
            Err(format!("Failed to resolve variable '{:?}'", varname))
        }
    }

    pub fn resolve_mut(&mut self, varname: &String) -> Result<&mut Environment, String> {
        if self.variables.contains_key(varname) {
            Ok(self)
        } else if let Some(box_parent) = self.parent.as_mut() {
            box_parent.resolve_mut(varname)
        } else {
            Err(format!("Failed to resolve mutable variable '{:?}'", varname))
        }
    }
}
