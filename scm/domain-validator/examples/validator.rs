//! Basic `Validator` usage example.

use edge_domain_validator::{AlwaysValid, Validator, ValidatorError};

struct Config {
    value: u32,
}

impl Validator for Config {
    fn validate(&self) -> Result<(), ValidatorError> {
        if self.value > 0 {
            Ok(())
        } else {
            Err(ValidatorError::Invalid("value must be positive".into()))
        }
    }
}

fn main() {
    println!("{:?}", Config { value: 1 }.validate());
    println!("{:?}", Config { value: 0 }.validate());
    println!("{:?}", AlwaysValid.validate());
}
