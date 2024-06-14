use {
    getset::{CopyGetters, Getters},
    serde::{de::DeserializeOwned, Deserialize, Serialize},
    std::{
        fmt::{Debug, Display, Write},
        rc::Rc,
    },
};

/// This struct represents a links object that MAY contain the following members:
///
/// - about: a link that leads to further details about this particular occurrence of the problem. When dereferenced, this URI SHOULD return a human-readable description of the error.
/// - type: a link that identifies the type of error that this particular error is an instance of. This URI SHOULD be dereferenceable to a human-readable explanation of the general error.
#[derive(
    PartialEq, Clone, Default, Deserialize, Serialize, Getters, Debug,
)]
#[getset(get = "pub with_prefix")]
pub struct ErrorLinks {
    #[serde(skip_serializing_if = "Option::is_none")]
    about: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<String>,
}
// ────────────────────────────────────────────────────────────
// vim: filetype=rust syntax=rust softtabstop=4 tabstop=4 shiftwidth=4 textwidth=79 fileencoding=utf-8 expandtab
// code: language=rust insertSpaces=true tabSize=4
