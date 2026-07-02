//! Constructors and `ContextManager` impl for `MapContextManager`.

use crate::api::ContextManager;
use crate::api::PromptError;
use crate::api::{
    ClearVariablesRequest, CompletenessRequest, CompletenessResponse, ContextBuildRequest,
    ContextBuildResponse, RegisterVariableRequest, VariableLookupRequest, VariableLookupResponse,
};
use crate::api::{MapContextManager, RenderContext, Variable};

impl MapContextManager {
    /// Construct an empty context manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of registered variables.
    pub fn len(&self) -> usize {
        self.variables.len()
    }

    /// Whether no variables are registered.
    pub fn is_empty(&self) -> bool {
        self.variables.is_empty()
    }
}

impl ContextManager for MapContextManager {
    fn register_variable(&mut self, req: RegisterVariableRequest<'_>) -> Result<(), PromptError> {
        if req.name.is_empty() {
            return Err(PromptError::InvalidValue {
                variable_name: req.name,
                reason: "variable name must not be empty".to_string(),
            });
        }
        self.variables.insert(req.name, req.var.clone());
        Ok(())
    }

    fn get_variable(
        &self,
        req: VariableLookupRequest<'_>,
    ) -> Result<VariableLookupResponse<'_>, PromptError> {
        Ok(VariableLookupResponse {
            variable: self.variables.get(req.name),
        })
    }

    fn build_context(
        &self,
        _req: ContextBuildRequest,
    ) -> Result<ContextBuildResponse, PromptError> {
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
        Ok(ContextBuildResponse {
            variables: context.variables,
            metadata: context.metadata,
            template_id: context.template_id,
        })
    }

    fn clear(&mut self, _req: ClearVariablesRequest) -> Result<(), PromptError> {
        self.variables.clear();
        Ok(())
    }

    fn is_complete(&self, _req: CompletenessRequest) -> Result<CompletenessResponse, PromptError> {
        Ok(CompletenessResponse {
            complete: self.variables.values().all(Variable::is_satisfied),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::VariableKind;

    fn required(name: &str) -> Variable {
        Variable::new(name.to_string(), VariableKind::String)
    }

    #[test]
    fn test_register_variable_stores_value() {
        let mut mgr = MapContextManager::new();
        mgr.register_variable(RegisterVariableRequest {
            name: "a".to_string(),
            var: &required("a"),
        })
        .expect("register should succeed");
        assert!(mgr
            .get_variable(VariableLookupRequest { name: "a" })
            .expect("get ok")
            .variable
            .is_some());
    }

    #[test]
    fn test_register_variable_rejects_empty_name() {
        let mut mgr = MapContextManager::new();
        assert!(mgr
            .register_variable(RegisterVariableRequest {
                name: String::new(),
                var: &required("a"),
            })
            .is_err());
    }

    #[test]
    fn test_build_context_errors_when_required_missing() {
        let mut mgr = MapContextManager::new();
        mgr.register_variable(RegisterVariableRequest {
            name: "a".to_string(),
            var: &required("a"),
        })
        .expect("register should succeed");
        assert!(mgr.build_context(ContextBuildRequest).is_err());
    }

    #[test]
    fn test_is_complete_false_with_unsatisfied_required() {
        let mut mgr = MapContextManager::new();
        mgr.register_variable(RegisterVariableRequest {
            name: "a".to_string(),
            var: &required("a"),
        })
        .expect("register should succeed");
        assert!(!mgr.is_complete(CompletenessRequest).expect("ok").complete);
    }

    #[test]
    fn test_clear_removes_all_variables() {
        let mut mgr = MapContextManager::new();
        mgr.register_variable(RegisterVariableRequest {
            name: "a".to_string(),
            var: &required("a"),
        })
        .expect("register should succeed");
        mgr.clear(ClearVariablesRequest).expect("clear ok");
        assert!(mgr.is_empty());
    }

    /// @covers: len
    #[test]
    fn test_len_reflects_registered_count() {
        let mut mgr = MapContextManager::new();
        mgr.register_variable(RegisterVariableRequest {
            name: "a".to_string(),
            var: &required("a"),
        })
        .expect("register should succeed");
        assert_eq!(mgr.len(), 1);
    }

    /// @covers: is_empty
    #[test]
    fn test_is_empty_true_for_new_manager() {
        assert!(MapContextManager::new().is_empty());
    }
}
