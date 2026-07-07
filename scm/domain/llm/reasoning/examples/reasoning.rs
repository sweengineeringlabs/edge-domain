//! Basic `edge-llm-reasoning` usage example.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_reasoning::{LinearReasoning, ReasonRequest, Reasoning, ReasoningPattern};
use futures::executor::block_on;

fn main() {
    let reasoner = LinearReasoning::new(ReasoningPattern::ChainOfThought);
    println!("pattern: {}", reasoner.pattern().as_str());

    match block_on(reasoner.reason(ReasonRequest {
        problem: "how do I ship this?",
        pattern: ReasoningPattern::ChainOfThought,
    })) {
        Ok(response) => {
            let process = response.process;
            println!("steps: {}", process.step_count());
            println!("complete: {}", process.is_complete);
            println!("avg confidence: {:.2}", process.average_confidence());
        }
        Err(err) => println!("reasoning failed: {}", err.message()),
    }
}
