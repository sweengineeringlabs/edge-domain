//! `Domain` — factory type for domain building-block constructors.

/// Domain building-block factory.
///
/// All constructors for domain infrastructure types live here as static
/// methods. Consumers call `Domain::echo_handler(...)` rather than
/// importing free functions.
pub struct Domain;
