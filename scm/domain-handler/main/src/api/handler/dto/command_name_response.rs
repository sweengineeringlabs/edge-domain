//! [`CommandNameResponse`] — result of [`Command::name`](crate::api::handler::traits::Command::name).

/// Stable name identifying a command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandNameResponse {
    /// The command's stable name.
    pub name: String,
}
