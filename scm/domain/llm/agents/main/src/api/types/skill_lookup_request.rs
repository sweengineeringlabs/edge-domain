/// Request for [`Agent::skill`](crate::api::traits::Agent::skill).
#[derive(Debug, Clone, Copy)]
pub struct SkillLookupRequest<'a> {
    /// Name of the skill to look up.
    pub name: &'a str,
}
