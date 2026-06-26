//! Basic example: sequentially execute steps with shared context mutation

use edge_domain_pipeline::{PipelineBuilder, PipelineConfig, PipelineSvc, Step};
use std::sync::Arc;
use std::time::Duration;

struct IncrementStep(i32);

#[async_trait::async_trait]
impl Step<i32, String> for IncrementStep {
    async fn execute(&self, ctx: &mut i32) -> Result<(), String> {
        *ctx += self.0;
        println!("Incremented by {} → {}", self.0, *ctx);
        Ok(())
    }
    fn name(&self) -> &str { "increment" }
}

struct MultiplyStep(i32);

#[async_trait::async_trait]
impl Step<i32, String> for MultiplyStep {
    async fn execute(&self, ctx: &mut i32) -> Result<(), String> {
        *ctx *= self.0;
        println!("Multiplied by {} → {}", self.0, *ctx);
        Ok(())
    }
    fn name(&self) -> &str { "multiply" }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Basic Pipeline Example ===\n");

    // Build a pipeline: 5 → +3 → *2 → 16
    let steps: Vec<Arc<dyn Step<i32, String>>> = vec![
        Arc::new(IncrementStep(3)),
        Arc::new(MultiplyStep(2)),
    ];
    let config = PipelineConfig {
        timeout_per_step: Some(Duration::from_secs(10)),
        emit_lifecycle_events: false,
        abort_on_error: true,
    };
    let pipeline = PipelineSvc::build(PipelineBuilder { steps, config, event_bus: None });

    let mut context = 5;
    println!("Initial context: {}", context);
    println!("\nExecuting pipeline...");

    pipeline.run(&mut context).await.map_err(|e| e.to_string())?;

    println!("\nFinal context: {}", context);
    println!("Expected: 16 (5 + 3 = 8, 8 * 2 = 16)");

    Ok(())
}
