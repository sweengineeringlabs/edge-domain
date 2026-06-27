//! [`Token`] — opaque bearer / access token.

/// An opaque bearer or access token passed on outbound requests or extracted
/// from inbound ones.
///
/// Deliberately keeps the inner value private so callers cannot accidentally
/// log or serialise raw token bytes. Use [`Token::as_str`] only at the point
/// of transmission.
#[derive(Clone, PartialEq, Eq)]
pub struct Token(pub(crate) String);
