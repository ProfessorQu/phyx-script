use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use nannou::color::*;
use palette::named::from_str;

use crate::{runtime::values::RuntimeValue, simulation::{Object, Physics}};

use super::native_fns;

pub struct Environment {
    parent: Option<Box<Self>>,
    pub physics: Option<Rc<RefCell<Physics>>>,
    pub objects: Option<Rc<RefCell<Vec<Object>>>>,
    variables: HashMap<String, RuntimeValue>
}

impl Environment {
    pub fn new(parent: Self) -> Self {
        Self {
            parent: Some(Box::new(parent)),
            physics: None,
            objects: None,
            variables: HashMap::new()
        }
    }

    pub fn new_global() -> Self {
        let physics = Some(Rc::new(RefCell::new(Physics::new())));
        let objects = Some(Rc::new(RefCell::new(vec![])));

        let mut env = Self {
            parent: None,
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
        env.declare_var("range".to_string(), RuntimeValue::NativeFn(native_fns::range)).expect("'range' already declared");

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

    pub fn resolve_objects(&self) -> Result<Rc<RefCell<Vec<Object>>>, String> {
        if let Some(objects) = &self.objects {
            Ok(objects.clone())
        } else if let Some(parent) = &self.parent {
            parent.resolve_objects()
        } else {
            Err("Failed to resolve objects".to_string())
        }
    }

    pub fn resolve_physics(&self) -> Result<Rc<RefCell<Physics>>, String> {
        if let Some(physics) = &self.physics {
            Ok(physics.clone())
        } else if let Some(parent) = &self.parent {
            parent.resolve_physics()
        } else {
            Err("Failed to resolve physics".to_string())
        }
    }
}

impl Debug for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Environment\n")?;
        f.write_str(format!("{:?}", self.variables).as_str())
    }
}

impl Clone for Environment {
    fn clone(&self) -> Self {
        Self {
            parent: self.parent.clone(),
            physics: None,
            objects: None,
            variables: self.variables.clone()
        }
    }
}
