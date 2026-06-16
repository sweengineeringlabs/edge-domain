//! Basic `Policy` usage example.

use edge_domain_policy::{CompositePolicy, Policy, PolicyFactory, PolicyViolation};

struct Factories;
impl PolicyFactory for Factories {}

struct MaxLength(usize);

impl Policy for MaxLength {
    type Input = String;
    fn name(&self) -> &'static str { "max-length" }
    fn evaluate(&self, s: &String) -> Result<(), PolicyViolation> {
        if s.len() <= self.0 {
            Ok(())
        } else {
            Err(PolicyViolation::new("max-length", format!("exceeds {}", self.0)))
        }
    }
}

fn main() {
    let policy: CompositePolicy<String> = Factories::composite::<String>()
        .with(Box::new(MaxLength(5)));
    println!("{:?}", policy.evaluate(&"hi".to_string()));
    println!("{:?}", policy.evaluate(&"too long".to_string()));
}
