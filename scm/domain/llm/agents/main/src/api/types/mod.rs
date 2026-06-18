//! Agent domain value types.
pub mod agent_endpoint;
pub mod agent_metadata;
pub mod agent_metadata_builder;
pub mod skill_metadata;
pub mod skill_metadata_builder;

pub use agent_endpoint::AgentEndpoint;
pub use agent_metadata::AgentMetadata;
pub use agent_metadata_builder::AgentMetadataBuilder;
pub use skill_metadata::SkillMetadata;
pub use skill_metadata_builder::SkillMetadataBuilder;

// LLM agent types (merged from edge_llm_agent)
pub mod agent_lifecycle_error;
pub mod agent_state;
pub mod cache_control;
pub mod content_part;
pub mod input_output_schema;
pub mod message;
pub mod message_builder;
pub mod message_content;
pub mod parameter_documentation;
pub mod parameter_documentation_builder;
pub mod role;
pub mod tool_call;
pub mod tool_choice;
pub mod validation_error;

pub use agent_lifecycle_error::AgentLifecycleError;
pub use agent_state::AgentState;
pub use cache_control::CacheControl;
pub use content_part::ContentPart;
pub use input_output_schema::InputOutputSchema;
pub use message::Message;
pub use message_builder::MessageBuilder;
pub use message_content::MessageContent;
pub use parameter_documentation::ParameterDocumentation;
pub use parameter_documentation_builder::ParameterDocumentationBuilder;
pub use role::Role;
pub use tool_call::ToolCall;
pub use tool_choice::ToolChoice;
pub use validation_error::ValidationError;
