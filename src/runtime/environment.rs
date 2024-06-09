use std::collections::HashMap;

use crate::runtime::values::RuntimeValue;

pub struct Environment {
    parent: Box<Option<Environment>>,
    variables: HashMap<String, RuntimeValue>
}

impl Environment {
    pub fn new(parent: Option<Environment>) -> Self {
        Self {
            parent: Box::new(parent),
            variables: HashMap::new()
        }
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
        } else if let Some(parent_env) = self.parent.as_ref() {
            parent_env.resolve(varname)
        } else {
            Err(format!("Failed to resolve variable '{:?}'", varname))
        }
    }

    pub fn resolve_mut(&mut self, varname: &String) -> Result<&mut Environment, String> {
        if self.variables.contains_key(varname) {
            Ok(self)
        } else if let Some(parent_env) = self.parent.as_mut() {
            parent_env.resolve_mut(varname)
        } else {
            Err(format!("Failed to resolve variable '{:?}'", varname))
        }
    }
}
