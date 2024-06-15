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
pub struct Links {
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
pub struct Source {
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
/// Represents JSON API Error object.
#[derive(
    PartialEq, Clone, Default, Deserialize, Serialize, Getters, Debug,
)]
#[getset(get = "pub with_prefix")]
pub struct Error {
    /// the HTTP status code applicable to this problem, expressed as a string value.
    /// This SHOULD be provided.
    status: String,
    /// a short, human-readable summary of the problem that SHOULD NOT change
    /// from occurrence to occurrence of the problem, except for purposes of localization.
    title: String,
    /// a unique identifier for this particular occurrence of the problem.
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    /// an application-specific error code, expressed as a string value.
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    /// a human-readable explanation specific to this occurrence of the problem.
    /// Like title, this field’s value can be localized.
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
    /// a meta object containing non-standard meta-information about the error
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<serde_json::Value>,
    /// a links object for failure responses
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<Links>,
    /// an object containing references to the primary source of the error
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Source>,
}

// ────────────────────────────────────────────────────────────
// vim: filetype=rust syntax=rust softtabstop=4 tabstop=4 shiftwidth=4 textwidth=79 fileencoding=utf-8 expandtab
// code: language=rust insertSpaces=true tabSize=4
