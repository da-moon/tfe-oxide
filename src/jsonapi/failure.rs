use {
    getset::{CopyGetters, Getters},
    serde::{de::DeserializeOwned, Deserialize, Serialize},
    std::{
        fmt::{Debug, Display, Write},
        rc::Rc,
    },
};

/// This struct represents a links object for failure responses.
#[derive(
    PartialEq, Clone, Default, Deserialize, Serialize, Getters, Debug,
)]
#[getset(get = "pub with_prefix")]
pub struct ErrorLinks {
    /// a link that leads to further details about this particular occurrence of the problem.
    /// When dereferenced, this URI SHOULD return a human-readable description of the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    about: Option<String>,
    /// a link that identifies the type of error that this particular error is an instance of.
    /// This URI SHOULD be dereferenceable to a human-readable explanation of the general error.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<String>,
}
/// This struct represents an object containing references to the primary source of the error.
#[derive(
    PartialEq, Clone, Default, Deserialize, Serialize, Getters, Debug,
)]
#[getset(get = "pub with_prefix")]
pub struct ErrorSource {
    /// a JSON Pointer [RFC6901](https://datatracker.ietf.org/doc/html/rfc6901) to the value in the request document that caused the error [e.g. "/data" for a primary data object, or "/data/attributes/title" for a specific attribute].
    /// This MUST point to a value in the request document that exists;
    /// if it doesn’t, the client SHOULD simply ignore the pointer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pointer: Option<String>,
    /// a string indicating which URI query parameter caused the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    parameter: Option<String>,
    /// a string indicating the name of a single request header which caused the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<String>,
}
// ────────────────────────────────────────────────────────────
// vim: filetype=rust syntax=rust softtabstop=4 tabstop=4 shiftwidth=4 textwidth=79 fileencoding=utf-8 expandtab
// code: language=rust insertSpaces=true tabSize=4
