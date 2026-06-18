//! [`Policy`] impl for [`CompositePolicy`].

use crate::api::Policy;
use crate::api::PolicyFactory;
use crate::api::CompositePolicy;
use crate::api::PolicyViolation;

#[expect(
    dead_code,
    reason = "SEA core/ structural anchor — not constructed anywhere"
)]
pub(crate) struct DefaultCompositePolicy;

impl<I: Send + Sync + 'static> Policy for CompositePolicy<I> {
    type Input = I;

    fn name(&self) -> &'static str {
        self.name
    }

    fn evaluate(&self, input: &I) -> Result<(), PolicyViolation> {
        for policy in &self.policies {
            policy.evaluate(input)?;
        }
        Ok(())
    }
}

impl PolicyFactory for DefaultCompositePolicy {}
