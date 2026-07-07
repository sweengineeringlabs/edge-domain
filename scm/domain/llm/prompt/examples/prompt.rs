//! Basic `edge-llm-prompt` usage example.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{
    Prompt, PromptMetadataBuilder, PromptMetadataRequest, PromptVariableKindRequest,
    RenderContext, RenderRequest, StaticPrompt, Variable, VariableBuilder, VariableKind,
};

fn main() {
    let variable = VariableBuilder::new()
        .name("name".to_string())
        .var_type(VariableKind::String)
        .build();

    let metadata = PromptMetadataBuilder::new()
        .id("greet".to_string())
        .name("Greeting".to_string())
        .version("1".to_string())
        .variables(vec![variable])
        .build();

    let prompt = StaticPrompt::new("Hello {{name}}".to_string(), metadata);
    println!(
        "template id: {}",
        prompt
            .metadata(PromptMetadataRequest)
            .expect("metadata ok")
            .id
    );
    println!(
        "name is: {:?}",
        prompt
            .variable_kind(PromptVariableKindRequest { name: "name" })
            .expect("variable_kind ok")
            .kind
    );

    let context = RenderContext::new().with_variable("name".to_string(), serde_json::json!("Ada"));
    match futures::executor::block_on(prompt.render(RenderRequest { context: &context })) {
        Ok(response) => println!("rendered: {}", response.rendered),
        Err(error) => println!("render failed: {}", error.message()),
    }

    // A registered variable round-trips through the value vocabulary.
    let _typed: Variable = VariableBuilder::new()
        .name("topic".to_string())
        .var_type(VariableKind::String)
        .build();
}
