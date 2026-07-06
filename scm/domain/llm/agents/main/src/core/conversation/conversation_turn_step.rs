//! `DefaultConversationTurnStep` — bridges one conversation turn into
//! `edge-domain-pipeline`'s foreign `Step` contract.

use std::sync::Arc;

use async_trait::async_trait;
use edge_domain_handler::HandlerContext;
use edge_pipeline::{ContextMutationRequest, Step};
use edge_llm_complete::{CompleteRequest, CompletionRequest, FinishReason};
use edge_llm_provider::{CompleterRequest, ModelInfoLookupRequest};

use crate::api::{
    AgentError, AgentProviderRequest, CacheControl, ContentPart, Message, MessageContent,
    OwnedHandlerContext, Role, SkillExecutionRequest, ToolCall,
};
use crate::core::conversation::conversation_state::ConversationState;

pub(super) struct DefaultConversationTurnStep {
    pub(super) agent: Arc<dyn crate::api::Agent>,
    pub(super) handler_context: OwnedHandlerContext,
}

#[async_trait]
impl Step for DefaultConversationTurnStep {
    type Ctx = ConversationState;
    type ExecutionError = AgentError;

    async fn execute(
        &self,
        req: ContextMutationRequest<'_, ConversationState>,
    ) -> Result<(), AgentError> {
        let ctx = req.ctx;
        if ctx.terminated {
            return Ok(());
        }

        let provider = self.agent.provider(AgentProviderRequest)?.provider;
        let model = provider
            .model_info(ModelInfoLookupRequest)
            .map_err(|e| AgentError::ExecutionFailed(e.message()))?
            .info
            .id
            .clone();
        let completer = provider
            .completer(CompleterRequest)
            .map_err(|e| AgentError::ExecutionFailed(e.message()))?
            .completer;

        let complete_messages: Vec<edge_llm_complete::Message> =
            ctx.messages.iter().map(Self::to_complete_message).collect();
        let request = CompletionRequest::new(model, complete_messages);

        let completion = completer
            .complete(CompleteRequest { request: &request })
            .await
            .map_err(|e| AgentError::ExecutionFailed(e.to_string()))?;

        ctx.messages.push(Self::from_completion(&completion));
        ctx.turns_taken += 1;

        if completion.finish_reason != FinishReason::ToolCalls || completion.tool_calls.is_empty() {
            ctx.terminated = true;
            return Ok(());
        }

        // Every tool call requested in this turn is executed, one after another (not
        // concurrently — turns and the skill calls within them are both sequential in
        // this design), and each result is appended to history before the next turn.
        for call in &completion.tool_calls {
            let handler_ctx = HandlerContext {
                security: &self.handler_context.security,
                commands: self.handler_context.commands.as_ref(),
                observer: self.handler_context.observer.as_ref(),
            };
            let result = self
                .agent
                .execute_skill(SkillExecutionRequest {
                    skill_name: &call.name,
                    input: call.arguments.clone(),
                    ctx: handler_ctx,
                })
                .await?;

            ctx.messages
                .push(Message::tool(result.output, call.id.clone()));
        }
        Ok(())
    }
}

impl DefaultConversationTurnStep {
    /// Convert this crate's own [`Message`] into `edge_llm_complete::Message`. Plain
    /// associated functions, not `From`/`TryFrom` — both `Message` types and the trait
    /// would be foreign to at least one crate in every combination, so the orphan rule
    /// blocks any impl.
    fn to_complete_message(msg: &Message) -> edge_llm_complete::Message {
        edge_llm_complete::Message {
            role: Self::to_complete_role(msg.role),
            content: Self::to_complete_content(&msg.content),
            name: msg.name.clone(),
            tool_call_id: msg.tool_call_id.clone(),
            tool_calls: msg
                .tool_calls
                .iter()
                .map(Self::to_complete_tool_call)
                .collect(),
            cache_control: msg
                .cache_control
                .as_ref()
                .map(Self::to_complete_cache_control),
        }
    }

    fn to_complete_role(role: Role) -> edge_llm_complete::Role {
        match role {
            Role::System => edge_llm_complete::Role::System,
            Role::User => edge_llm_complete::Role::User,
            Role::Assistant => edge_llm_complete::Role::Assistant,
            Role::Tool => edge_llm_complete::Role::Tool,
        }
    }

    fn to_complete_content(content: &MessageContent) -> edge_llm_complete::MessageContent {
        match content {
            MessageContent::Text(text) => edge_llm_complete::MessageContent::Text(text.clone()),
            MessageContent::Parts(parts) => edge_llm_complete::MessageContent::Parts(
                parts.iter().map(Self::to_complete_part).collect(),
            ),
        }
    }

    fn to_complete_part(part: &ContentPart) -> edge_llm_complete::ContentPart {
        match part {
            ContentPart::Text { text } => {
                edge_llm_complete::ContentPart::Text { text: text.clone() }
            }
            ContentPart::ImageUrl { image_url } => edge_llm_complete::ContentPart::ImageUrl {
                image_url: Box::new(edge_llm_complete::ImageUrl {
                    url: image_url.clone(),
                    detail: None,
                }),
            },
            ContentPart::ImageBase64 { data, media_type } => {
                edge_llm_complete::ContentPart::ImageBase64 {
                    data: data.clone(),
                    media_type: media_type.clone(),
                }
            }
        }
    }

    fn to_complete_tool_call(call: &ToolCall) -> edge_llm_complete::ToolCall {
        edge_llm_complete::ToolCall {
            id: call.id.clone(),
            name: call.name.clone(),
            arguments: call.arguments.clone(),
        }
    }

    fn to_complete_cache_control(cc: &CacheControl) -> edge_llm_complete::CacheControl {
        edge_llm_complete::CacheControl {
            cache_type: cc.cache_type.clone(),
        }
    }

    fn from_complete_tool_call(call: &edge_llm_complete::ToolCall) -> ToolCall {
        ToolCall {
            id: call.id.clone(),
            name: call.name.clone(),
            arguments: call.arguments.clone(),
        }
    }

    /// Build the assistant turn [`Message`] from a completion response.
    ///
    /// Stated assumption: this crate's own [`MessageContent`] has no `Empty` variant
    /// (unlike `edge_llm_complete::MessageContent`), so a `None` completion content maps
    /// to `MessageContent::Text(String::new())`.
    fn from_completion(resp: &edge_llm_complete::CompletionResponse) -> Message {
        Message {
            role: Role::Assistant,
            content: MessageContent::Text(resp.content.clone().unwrap_or_default()),
            name: None,
            tool_call_id: None,
            tool_calls: resp
                .tool_calls
                .iter()
                .map(Self::from_complete_tool_call)
                .collect(),
            cache_control: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{
        AgentDescriptionRequest, AgentDescriptionResponse, AgentIdRequest, AgentIdResponse,
        AgentNameRequest, AgentNameResponse, AgentProviderResponse, AgentSkillsRequest,
        AgentSkillsResponse, SkillExecutionResponse,
    };
    use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
    use edge_domain_observer::StdObserveFactory;
    use edge_security_runtime::SecurityContext;
    use futures::executor::block_on;

    struct DefaultConversationTurnStepPanicsIfCalledAgent;

    #[async_trait]
    impl crate::api::Agent for DefaultConversationTurnStepPanicsIfCalledAgent {
        fn id(&self, _req: AgentIdRequest) -> Result<AgentIdResponse, AgentError> {
            Ok(AgentIdResponse {
                id: "panics".to_string(),
            })
        }
        fn name(&self, _req: AgentNameRequest) -> Result<AgentNameResponse, AgentError> {
            Ok(AgentNameResponse {
                name: "panics".to_string(),
            })
        }
        fn description(
            &self,
            _req: AgentDescriptionRequest,
        ) -> Result<AgentDescriptionResponse, AgentError> {
            Ok(AgentDescriptionResponse {
                description: "panics if called".to_string(),
            })
        }
        async fn execute_skill(
            &self,
            _req: SkillExecutionRequest<'_>,
        ) -> Result<SkillExecutionResponse, AgentError> {
            panic!("execute_skill should not be called on a terminated conversation")
        }
        fn skills(&self, _req: AgentSkillsRequest) -> Result<AgentSkillsResponse, AgentError> {
            Ok(AgentSkillsResponse { skills: vec![] })
        }
        fn provider(
            &self,
            _req: AgentProviderRequest,
        ) -> Result<AgentProviderResponse, AgentError> {
            panic!("provider() should not be called on a terminated conversation")
        }
    }

    fn owned_handler_context() -> OwnedHandlerContext {
        OwnedHandlerContext {
            security: SecurityContext::unauthenticated(),
            commands: Arc::new(StdCommandBusFactory::direct()),
            observer: StdObserveFactory::noop_observer_context(),
        }
    }

    /// @covers: execute
    #[test]
    fn test_execute_terminated_context_is_noop_edge() {
        let step = DefaultConversationTurnStep {
            agent: Arc::new(DefaultConversationTurnStepPanicsIfCalledAgent),
            handler_context: owned_handler_context(),
        };
        let mut state = ConversationState {
            messages: vec![Message::user("hi")],
            turns_taken: 0,
            terminated: true,
        };
        let result = block_on(step.execute(ContextMutationRequest { ctx: &mut state }));
        assert!(result.is_ok());
        assert_eq!(state.messages.len(), 1);
    }

    // execute()'s happy/error paths (provider completion, skill dispatch, skill failure
    // propagation) are covered externally in tests/conversation_loop_e2e_test.rs — only
    // the terminated-is-noop edge case is tested inline above, since it's the one path
    // not exercised by any e2e scenario (a completed conversation's unused pipeline
    // slack turns must stay silent, not re-invoke the provider).

    /// @covers: to_complete_message
    #[test]
    fn test_to_complete_message_maps_all_fields_happy() {
        let msg = Message {
            role: Role::Assistant,
            content: MessageContent::Text("hi there".to_string()),
            name: Some("assistant-1".to_string()),
            tool_call_id: Some("call-9".to_string()),
            tool_calls: vec![ToolCall {
                id: "call-1".to_string(),
                name: "get_weather".to_string(),
                arguments: "{}".to_string(),
            }],
            cache_control: Some(CacheControl {
                cache_type: "ephemeral".to_string(),
            }),
        };
        let converted = DefaultConversationTurnStep::to_complete_message(&msg);
        assert_eq!(converted.role, edge_llm_complete::Role::Assistant);
        assert_eq!(
            converted.content,
            edge_llm_complete::MessageContent::Text("hi there".to_string())
        );
        assert_eq!(converted.name, Some("assistant-1".to_string()));
        assert_eq!(converted.tool_call_id, Some("call-9".to_string()));
        assert_eq!(converted.tool_calls.len(), 1);
        assert_eq!(converted.tool_calls[0].name, "get_weather");
        assert_eq!(
            converted.cache_control.map(|cc| cc.cache_type),
            Some("ephemeral".to_string())
        );
    }

    /// @covers: to_complete_message
    #[test]
    fn test_to_complete_message_no_optional_fields_edge() {
        let msg = Message {
            role: Role::User,
            content: MessageContent::Text("hi".to_string()),
            name: None,
            tool_call_id: None,
            tool_calls: vec![],
            cache_control: None,
        };
        let converted = DefaultConversationTurnStep::to_complete_message(&msg);
        assert_eq!(converted.name, None);
        assert_eq!(converted.tool_call_id, None);
        assert!(converted.tool_calls.is_empty());
        assert_eq!(converted.cache_control, None);
    }

    /// @covers: to_complete_part
    #[test]
    fn test_to_complete_part_text_happy() {
        let part = ContentPart::Text {
            text: "hello".to_string(),
        };
        let converted = DefaultConversationTurnStep::to_complete_part(&part);
        assert!(matches!(converted, edge_llm_complete::ContentPart::Text { text } if text == "hello"));
    }

    /// @covers: to_complete_part
    #[test]
    fn test_to_complete_part_image_url_happy() {
        let part = ContentPart::ImageUrl {
            image_url: "https://example.com/x.png".to_string(),
        };
        let converted = DefaultConversationTurnStep::to_complete_part(&part);
        match converted {
            edge_llm_complete::ContentPart::ImageUrl { image_url } => {
                assert_eq!(image_url.url, "https://example.com/x.png");
                assert_eq!(image_url.detail, None);
            }
            other => panic!("expected ImageUrl, got {other:?}"),
        }
    }

    /// @covers: to_complete_part
    #[test]
    fn test_to_complete_part_image_base64_edge() {
        let part = ContentPart::ImageBase64 {
            data: "base64data".to_string(),
            media_type: "image/png".to_string(),
        };
        let converted = DefaultConversationTurnStep::to_complete_part(&part);
        match converted {
            edge_llm_complete::ContentPart::ImageBase64 { data, media_type } => {
                assert_eq!(data, "base64data");
                assert_eq!(media_type, "image/png");
            }
            other => panic!("expected ImageBase64, got {other:?}"),
        }
    }

    /// @covers: to_complete_role
    #[test]
    fn test_to_complete_role_maps_every_variant_happy() {
        assert_eq!(
            DefaultConversationTurnStep::to_complete_role(Role::System),
            edge_llm_complete::Role::System
        );
        assert_eq!(
            DefaultConversationTurnStep::to_complete_role(Role::User),
            edge_llm_complete::Role::User
        );
        assert_eq!(
            DefaultConversationTurnStep::to_complete_role(Role::Assistant),
            edge_llm_complete::Role::Assistant
        );
        assert_eq!(
            DefaultConversationTurnStep::to_complete_role(Role::Tool),
            edge_llm_complete::Role::Tool
        );
    }

    /// @covers: to_complete_content
    #[test]
    fn test_to_complete_content_text_round_trips_happy() {
        let content = MessageContent::Text("hello".to_string());
        assert_eq!(
            DefaultConversationTurnStep::to_complete_content(&content),
            edge_llm_complete::MessageContent::Text("hello".to_string())
        );
    }

    /// @covers: to_complete_content
    #[test]
    fn test_to_complete_content_parts_round_trips_edge() {
        let content = MessageContent::Parts(vec![ContentPart::Text {
            text: "part".to_string(),
        }]);
        let converted = DefaultConversationTurnStep::to_complete_content(&content);
        assert!(
            matches!(converted, edge_llm_complete::MessageContent::Parts(parts) if parts.len() == 1)
        );
    }

    /// @covers: to_complete_tool_call
    #[test]
    fn test_to_complete_tool_call_maps_all_fields_happy() {
        let call = ToolCall {
            id: "id-1".to_string(),
            name: "get_weather".to_string(),
            arguments: "{}".to_string(),
        };
        let converted = DefaultConversationTurnStep::to_complete_tool_call(&call);
        assert_eq!(converted.id, "id-1");
        assert_eq!(converted.name, "get_weather");
        assert_eq!(converted.arguments, "{}");
    }

    /// @covers: from_complete_tool_call
    #[test]
    fn test_from_complete_tool_call_maps_all_fields_happy() {
        let call = edge_llm_complete::ToolCall {
            id: "id-2".to_string(),
            name: "get_time".to_string(),
            arguments: "{}".to_string(),
        };
        let converted = DefaultConversationTurnStep::from_complete_tool_call(&call);
        assert_eq!(converted.id, "id-2");
        assert_eq!(converted.name, "get_time");
        assert_eq!(converted.arguments, "{}");
    }

    /// @covers: to_complete_cache_control
    #[test]
    fn test_to_complete_cache_control_maps_type_happy() {
        let cc = CacheControl {
            cache_type: "ephemeral".to_string(),
        };
        assert_eq!(
            DefaultConversationTurnStep::to_complete_cache_control(&cc).cache_type,
            "ephemeral"
        );
    }

    /// @covers: from_completion
    #[test]
    fn test_from_completion_with_content_builds_text_message_happy() {
        let resp = edge_llm_complete::CompletionResponse {
            content: Some("hi there".to_string()),
            ..Default::default()
        };
        let msg = DefaultConversationTurnStep::from_completion(&resp);
        assert_eq!(msg.role, Role::Assistant);
        assert_eq!(msg.content, MessageContent::Text("hi there".to_string()));
        assert!(msg.tool_calls.is_empty());
    }

    /// @covers: from_completion
    #[test]
    fn test_from_completion_none_content_maps_to_empty_text_edge() {
        let resp = edge_llm_complete::CompletionResponse {
            content: None,
            ..Default::default()
        };
        let msg = DefaultConversationTurnStep::from_completion(&resp);
        assert_eq!(msg.content, MessageContent::Text(String::new()));
    }

    /// @covers: from_completion
    #[test]
    fn test_from_completion_with_tool_calls_maps_them_happy() {
        let resp = edge_llm_complete::CompletionResponse {
            content: None,
            tool_calls: vec![edge_llm_complete::ToolCall {
                id: "call-1".to_string(),
                name: "get_weather".to_string(),
                arguments: "{}".to_string(),
            }],
            ..Default::default()
        };
        let msg = DefaultConversationTurnStep::from_completion(&resp);
        assert_eq!(msg.tool_calls.len(), 1);
        assert_eq!(msg.tool_calls[0].name, "get_weather");
    }
}
