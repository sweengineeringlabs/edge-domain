//! [`Policy`] impl for [`CompositePolicy`].

use crate::api::policy::errors::PolicyViolation;
use crate::api::policy::traits::Policy;
use crate::api::policy::types::CompositePolicy;

#[expect(
    dead_code,
    reason = "SEA core/ structural anchor — CompositePolicy is generic, so the file needs a concrete type matching its name"
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
