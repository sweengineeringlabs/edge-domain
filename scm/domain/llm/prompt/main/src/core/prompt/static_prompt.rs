//! Constructors and `Prompt` impl for `StaticPrompt`.

use async_trait::async_trait;

use crate::api::Prompt;
use crate::api::PromptError;
use crate::api::{
    CacheBuildRequest, CacheBuildResponse, PromptMetadataRequest, PromptMetadataResponse,
    PromptVariableKindRequest, PromptVariableKindResponse, RenderRequest, RenderResponse,
    TemplateValidationRequest,
};
use crate::api::{PromptCache, PromptMetadata, RenderContext, StaticPrompt};

impl StaticPrompt {
    /// Construct a prompt from a template body and its metadata.
    pub fn new(template: String, metadata: PromptMetadata) -> Self {
        Self {
            template,
            metadata: Some(metadata),
        }
    }

    /// Render the template against `context`, substituting `{{name}}` tokens.
    ///
    /// Returns the rendered string and the list of placeholder names that had
    /// no matching variable in the context (used for completeness checks).
    pub(crate) fn substitute(&self, context: &RenderContext) -> (String, Vec<String>) {
        let variables = self
            .metadata
            .as_ref()
            .map(|m| m.variables.as_slice())
            .unwrap_or(&[]);
        let mut output = self.template.clone();
        let mut missing = Vec::new();
        for var in variables {
            let token = format!("{{{{{}}}}}", var.name);
            match context.get_variable(&var.name) {
                Some(value) => {
                    output = output.replace(&token, &value.display());
                }
                None if var.required => missing.push(var.name.clone()),
                None => {}
            }
        }
        (output, missing)
    }
}

#[async_trait]
impl Prompt for StaticPrompt {
    async fn render(&self, req: RenderRequest<'_>) -> Result<RenderResponse, PromptError> {
        self.validate(TemplateValidationRequest)?;
        let (rendered, missing) = self.substitute(req.context);
        if !missing.is_empty() {
            return Err(PromptError::IncompleteContext {
                missing_variables: missing,
            });
        }
        Ok(RenderResponse { rendered })
    }

    fn metadata(&self, _req: PromptMetadataRequest) -> Result<PromptMetadataResponse, PromptError> {
        let metadata = self.metadata.clone().unwrap_or_default();
        Ok(PromptMetadataResponse {
            id: metadata.id,
            name: metadata.name,
            version: metadata.version,
            variables: metadata.variables,
            description: metadata.description,
            base_token_count: metadata.base_token_count,
            tags: metadata.tags,
        })
    }

    fn validate(&self, _req: TemplateValidationRequest) -> Result<(), PromptError> {
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

    fn variable_kind(
        &self,
        req: PromptVariableKindRequest<'_>,
    ) -> Result<PromptVariableKindResponse, PromptError> {
        let kind = self
            .metadata
            .as_ref()
            .and_then(|m| m.variables.iter().find(|v| v.name == req.name))
            .map(|v| v.var_type);
        Ok(PromptVariableKindResponse { kind })
    }

    fn cache(&self, req: CacheBuildRequest<'_>) -> Result<CacheBuildResponse, PromptError> {
        let meta_id = self.metadata.as_ref().map(|m| m.id.as_str()).unwrap_or("");
        let key = format!(
            "{}::{}",
            meta_id,
            req.context.template_id.as_deref().unwrap_or("")
        );
        let token_count = req.rendered.len();
        let cache = PromptCache::new(key, req.rendered, token_count);
        Ok(CacheBuildResponse {
            key: cache.key,
            rendered: cache.rendered,
            token_count: cache.token_count,
            created_at: cache.created_at,
            ttl_seconds: cache.ttl_seconds,
            hit_count: cache.hit_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{Variable, VariableKind};
    use futures::executor::block_on;

    fn prompt() -> StaticPrompt {
        let var = Variable::new("name".to_string(), VariableKind::String);
        let metadata = PromptMetadata::new(
            "greet".to_string(),
            "Greeting".to_string(),
            "1".to_string(),
            vec![var],
        );
        StaticPrompt::new("Hello {{name}}".to_string(), metadata)
    }

    #[test]
    fn test_new_stores_metadata() {
        let result = prompt()
            .metadata(PromptMetadataRequest)
            .expect("metadata ok");
        assert_eq!(result.id, "greet");
    }

    #[test]
    fn test_render_substitutes_variable() {
        let ctx = RenderContext::new().with_variable("name".to_string(), serde_json::json!("Ada"));
        let out = block_on(prompt().render(RenderRequest { context: &ctx }))
            .expect("render should succeed")
            .rendered;
        assert_eq!(out, "Hello Ada");
    }

    #[test]
    fn test_render_errors_on_missing_required() {
        let ctx = RenderContext::new();
        assert!(block_on(prompt().render(RenderRequest { context: &ctx })).is_err());
    }

    #[test]
    fn test_validate_rejects_unbalanced_braces() {
        let metadata =
            PromptMetadata::new("x".to_string(), "x".to_string(), "1".to_string(), vec![]);
        let p = StaticPrompt::new("Hello {{name}".to_string(), metadata);
        assert!(p.validate(TemplateValidationRequest).is_err());
    }

    #[test]
    fn test_variable_kind_reports_declared_type() {
        let result = prompt()
            .variable_kind(PromptVariableKindRequest { name: "name" })
            .expect("variable_kind ok");
        assert_eq!(result.kind, Some(VariableKind::String));
    }

    #[test]
    fn test_cache_builds_entry_with_rendered_text() {
        let ctx = RenderContext::new();
        let result = prompt()
            .cache(CacheBuildRequest {
                context: &ctx,
                rendered: "Hello Ada".to_string(),
            })
            .expect("cache ok");
        assert_eq!(result.rendered, "Hello Ada");
    }
}
