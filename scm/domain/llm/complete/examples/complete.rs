//! Basic `edge-llm-complete` usage example.

use edge_llm_complete::{
    CompleteBootstrap, Completer, EchoCompleter, Message, StdCompleteFactory,
};
use futures::executor::block_on;

fn main() {
    let req = StdCompleteFactory::request(
        "echo".to_string(),
        vec![Message::user("Hello, world!")],
    );

    let resp = block_on(EchoCompleter.complete(&req)).expect("completion failed");
    println!("response: {:?}", resp.content);
    println!("finish_reason: {:?}", resp.finish_reason);
    println!("usage: {:?}", resp.usage);
}
