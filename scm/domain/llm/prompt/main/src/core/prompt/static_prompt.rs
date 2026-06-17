//! `Prompt` impl for `StaticPrompt`.

use async_trait::async_trait;

use crate::api::Prompt;
use crate::api::PromptError;
use crate::api::{PromptCache, PromptMetadata, RenderContext, StaticPrompt, VariableType};

#[async_trait]
impl Prompt for StaticPrompt {
    async fn render(&self, context: &RenderContext) -> Result<String, PromptError> {
        self.validate()?;
        let (rendered, missing) = self.substitute(context);
        if !missing.is_empty() {
            return Err(PromptError::IncompleteContext {
                missing_variables: missing,
            });
        }
        Ok(rendered)
    }

    fn metadata(&self) -> PromptMetadata {
        self.metadata.clone()
    }

    fn validate(&self) -> Result<(), PromptError> {
        let opens = self.template.matches("{{").count();
        let closes = self.template.matches("}}").count();
        if opens != closes {
            return Err(PromptError::InvalidSyntax {
                message: format!(
                    "unbalanced placeholder braces: {} '{{{{' vs {} '}}}}'",
                    opens, closes
                ),
            });
        }
        Ok(())
    }

    fn variable_type(&self, name: &str) -> Option<VariableType> {
        self.metadata
            .variables
            .iter()
            .find(|v| v.name == name)
            .map(|v| v.var_type)
    }

    fn cache(&self, context: &RenderContext, rendered: String) -> PromptCache {
        let key = format!(
            "{}::{}",
            self.metadata.id,
            context.template_id.as_deref().unwrap_or("")
        );
        let token_count = rendered.len();
        PromptCache::new(key, rendered, token_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{Variable, VariableType};
    use futures::executor::block_on;

    fn prompt() -> StaticPrompt {
        let var = Variable::new("name".to_string(), VariableType::String);
        let metadata = PromptMetadata::new(
            "greet".to_string(),
            "Greeting".to_string(),
            "1".to_string(),
            vec![var],
        );
        StaticPrompt::new("Hello {{name}}".to_string(), metadata)
    }

    #[test]
    fn test_render_substitutes_variable() {
        let ctx = RenderContext::new().with_variable("name".to_string(), serde_json::json!("Ada"));
        let out = block_on(prompt().render(&ctx)).expect("render should succeed");
        assert_eq!(out, "Hello Ada");
    }

    #[test]
    fn test_render_errors_on_missing_required() {
        let ctx = RenderContext::new();
        assert!(block_on(prompt().render(&ctx)).is_err());
    }

    #[test]
    fn test_validate_rejects_unbalanced_braces() {
        let metadata =
            PromptMetadata::new("x".to_string(), "x".to_string(), "1".to_string(), vec![]);
        let p = StaticPrompt::new("Hello {{name}".to_string(), metadata);
        assert!(p.validate().is_err());
    }

    #[test]
    fn test_variable_type_reports_declared_type() {
        assert_eq!(prompt().variable_type("name"), Some(VariableType::String));
    }
}
