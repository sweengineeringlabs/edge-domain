//! Basic `edge-llm-prompt` usage example.

use edge_llm_prompt::{
    Prompt, PromptBootstrap, RenderContext, StdPromptFactory, Variable, VariableType,
};

fn main() {
    let variable = StdPromptFactory::variable_builder()
        .name("name".to_string())
        .var_type(VariableType::String)
        .build();

    let metadata = StdPromptFactory::prompt_metadata_builder()
        .id("greet".to_string())
        .name("Greeting".to_string())
        .version("1".to_string())
        .variables(vec![variable])
        .build();

    let prompt = StdPromptFactory::prompt("Hello {{name}}".to_string(), metadata);
    println!("template id: {}", prompt.metadata().id);
    println!("name is: {:?}", prompt.variable_type("name"));

    let context = RenderContext::new().with_variable("name".to_string(), serde_json::json!("Ada"));
    match futures::executor::block_on(prompt.render(&context)) {
        Ok(rendered) => println!("rendered: {}", rendered),
        Err(error) => println!("render failed: {}", error.message()),
    }

    // A registered variable round-trips through the value vocabulary.
    let _typed: Variable = StdPromptFactory::variable_builder()
        .name("topic".to_string())
        .var_type(VariableType::String)
        .build();
}
