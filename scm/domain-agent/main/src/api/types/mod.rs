// Existing domain-agent types
pub mod agent_metadata;
pub mod agent_metadata_builder;
pub mod skill_metadata;
pub mod skill_metadata_builder;

pub use agent_metadata::AgentMetadata;
pub use agent_metadata_builder::AgentMetadataBuilder;
pub use skill_metadata::SkillMetadata;
pub use skill_metadata_builder::SkillMetadataBuilder;

// LLM agent types (merged from edge_llm_agent)
pub mod agent_state;
pub mod agent_lifecycle_error;
pub mod parameter_documentation;
pub mod input_output_schema;
pub mod validation_error;
pub mod message_content;
pub mod content_part;
pub mod message;
pub mod role;
pub mod tool_choice;
pub mod tool_call;
pub mod cache_control;

pub use agent_state::AgentState;
pub use agent_lifecycle_error::AgentLifecycleError;
pub use parameter_documentation::ParameterDocumentation;
pub use input_output_schema::InputOutputSchema;
pub use validation_error::ValidationError;
pub use message_content::MessageContent;
pub use content_part::ContentPart;
pub use message::Message;
pub use role::Role;
pub use tool_choice::ToolChoice;
pub use tool_call::ToolCall;
pub use cache_control::CacheControl;
