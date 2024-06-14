use serde::{Deserialize, Serialize};

/// Custom enum that contains all the possible errors that may occur when making
/// API requests.
#[derive(Debug, PartialEq, Eq, thiserror::Error, miette::Diagnostic)]
pub enum Error {
    /// The request was made, but the server returned an unsuccessful status
    /// code, such as 404 or 503. In some cases, the response may contain a
    /// error detail from the server.
    #[error("`{canonical_reason}`")]
    #[diagnostic(code(core::response))]
    Response {
        /// short description of the error
        canonical_reason: String,
        /// HTTP status code
        status: Option<String>,
        /// Complete JSON response from the server
        body: Option<serde_json::Value>,
    },
}
// ────────────────────────────────────────────────────────────
// vim: filetype=rust syntax=rust softtabstop=4 tabstop=4 shiftwidth=4 textwidth=79 fileencoding=utf-8 expandtab
// code: language=rust insertSpaces=true tabSize=4
