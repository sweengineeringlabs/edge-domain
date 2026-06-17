//! `ContextManager` impl for `MapContextManager`.

use crate::api::ContextManager;
use crate::api::PromptError;
use crate::api::{MapContextManager, RenderContext, Variable};

impl ContextManager for MapContextManager {
    fn register_variable(&mut self, name: String, var: Variable) -> Result<(), PromptError> {
        if name.is_empty() {
            return Err(PromptError::InvalidValue {
                variable_name: name,
                reason: "variable name must not be empty".to_string(),
            });
        }
        self.variables.insert(name, var);
        Ok(())
    }

    fn get_variable(&self, name: &str) -> Option<&Variable> {
        self.variables.get(name)
    }

    fn build_context(&self) -> Result<RenderContext, PromptError> {
        let mut missing = Vec::new();
        let mut context = RenderContext::new();
        for (name, var) in &self.variables {
            match var.get_value() {
                Some(value) => {
                    context = context.with_variable(name.clone(), value.clone());
                }
                None if var.required => missing.push(name.clone()),
                None => {}
            }
        }
        if !missing.is_empty() {
            return Err(PromptError::IncompleteContext {
                missing_variables: missing,
            });
        }
        Ok(context)
    }

    fn clear(&mut self) {
        self.variables.clear();
    }

    fn is_complete(&self) -> bool {
        self.variables.values().all(Variable::is_satisfied)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::VariableType;

    fn required(name: &str) -> Variable {
        Variable::new(name.to_string(), VariableType::String)
    }

    #[test]
    fn test_register_variable_stores_value() {
        let mut mgr = MapContextManager::new();
        mgr.register_variable("a".to_string(), required("a"))
            .expect("register should succeed");
        assert!(mgr.get_variable("a").is_some());
    }

    #[test]
    fn test_register_variable_rejects_empty_name() {
        let mut mgr = MapContextManager::new();
        assert!(mgr.register_variable(String::new(), required("a")).is_err());
    }

    #[test]
    fn test_build_context_errors_when_required_missing() {
        let mut mgr = MapContextManager::new();
        mgr.register_variable("a".to_string(), required("a"))
            .expect("register should succeed");
        assert!(mgr.build_context().is_err());
    }

    #[test]
    fn test_is_complete_false_with_unsatisfied_required() {
        let mut mgr = MapContextManager::new();
        mgr.register_variable("a".to_string(), required("a"))
            .expect("register should succeed");
        assert!(!mgr.is_complete());
    }

    #[test]
    fn test_clear_removes_all_variables() {
        let mut mgr = MapContextManager::new();
        mgr.register_variable("a".to_string(), required("a"))
            .expect("register should succeed");
        mgr.clear();
        assert!(mgr.is_empty());
    }
}
