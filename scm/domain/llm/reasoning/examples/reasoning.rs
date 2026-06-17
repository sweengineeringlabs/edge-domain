//! Basic `edge-llm-reasoning` usage example.

use edge_llm_reasoning::{Reasoning, ReasoningFactory, ReasoningPattern, StdReasoningFactory};
use futures::executor::block_on;

fn main() {
    let reasoner = StdReasoningFactory::reasoning(ReasoningPattern::ChainOfThought);
    println!("pattern: {}", reasoner.pattern().as_str());

    match block_on(reasoner.reason("how do I ship this?", ReasoningPattern::ChainOfThought)) {
        Ok(process) => {
            println!("steps: {}", process.step_count());
            println!("complete: {}", process.is_complete);
            println!("avg confidence: {:.2}", process.average_confidence());
        }
        Err(err) => println!("reasoning failed: {}", err.message()),
    }
}
