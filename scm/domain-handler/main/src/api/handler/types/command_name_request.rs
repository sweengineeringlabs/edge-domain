//! [`CommandNameRequest`] — input for [`Command::name`](crate::api::handler::traits::Command::name).

/// Marker request; `name` takes no data beyond `&self`.
#[derive(Debug, Clone, Copy, Default)]
pub struct CommandNameRequest;
