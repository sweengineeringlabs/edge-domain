use serde::{Deserialize, Serialize};

/// Error taxonomy for OAuth token source initialization.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OauthTokenSourceError {
    /// The credential file could not be read.
    CredentialFileUnreadable(String),
    /// The credential file was read but its contents are malformed.
    MalformedCredentials(String),
}
