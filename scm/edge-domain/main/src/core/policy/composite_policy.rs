//! [`Policy`] impl for [`CompositePolicy`].

use crate::api::policy::traits::Policy;
use crate::api::policy::traits::PolicyFactory;
use crate::api::policy::types::CompositePolicy;
use crate::api::policy::types::PolicyViolation;

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
