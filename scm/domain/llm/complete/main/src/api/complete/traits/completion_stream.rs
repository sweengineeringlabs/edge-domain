//! `CompletionStream` — streaming output type alias.

use futures::stream::BoxStream;

use crate::api::complete::errors::CompleteError;
use crate::api::complete::types::StreamChunk;

/// Owned stream of incremental completion chunks.
pub type CompletionStream = BoxStream<'static, Result<StreamChunk, CompleteError>>;
