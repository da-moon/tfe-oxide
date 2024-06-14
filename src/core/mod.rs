/// This module implements possible error message(s) that might occur when
/// making API calls.
// TODO: maybe this should be private
pub mod errors;
/// This module implements HttpClient trait using reqwest
// TODO: maybe this should be private
pub mod reqwest;
// ────────────────────────────────────────────────────────────
use {
    serde::{de::DeserializeOwned, Serialize},
    std::fmt::Debug,
    std::marker::{Send, Sync},
};
// ────────────────────────────────────────────────────────────
/// This represents a error in this library.
pub type Error = errors::Error;
/// This represents client headers.
pub type Headers = std::collections::HashMap<String, String>;
/// This represents query section of an HTTP request.
pub type Query<'a> = std::collections::HashMap<&'a str, &'a str>;
/// This represents Reqwest client implementation of HttpClient trait
pub type ReqwestClient = crate::core::reqwest::Client;
/// This is used to build a Reqwest client.
pub type ReqwestClientBuilder = crate::core::reqwest::Builder;
// ────────────────────────────────────────────────────────────
/// This trait represents the interface to be implemented for an HTTP client,
/// which is kept separate from the implementation.
///
/// When a request doesn't need to pass parameters, the empty or default value
/// of the payload type should be passed, like `json!({})` or `Query::new()`.
/// This avoids using `Option<T>` because `serde_json::Value` itself may be null in other
/// different ways (`serde_json::Value::Null`, an empty `serde_json::Value::Object`...), so this removes
/// redundancy and edge cases (a `Some(serde_json::Value::Null), for example, doesn't make
/// much sense).
#[maybe_async::maybe_async]
pub trait HttpClient: Send + Clone + Debug {
    /// sends GET request
    async fn get<R, S>(
        &self,
        url: S,
        headers: Option<&Headers>,
        payload: Option<&Query>,
    ) -> Result<R, Error>
    where
        R: DeserializeOwned + Serialize + Debug,
        S: AsRef<str> + Sync + Send;

    /// sends POST request
    async fn post<R, S, T>(
        &self,
        url: S,
        headers: Option<&Headers>,
        payload: T,
    ) -> Result<R, Error>
    where
        R: DeserializeOwned + Serialize + Debug,
        S: AsRef<str> + Sync + Send,
        T: Serialize + Debug + Send + Sync;

    /// sends PUT request
    async fn put<R, S, T>(
        &self,
        url: S,
        headers: Option<&Headers>,
        payload: T,
    ) -> Result<R, Error>
    where
        R: DeserializeOwned + Serialize + Debug,
        S: AsRef<str> + Sync + Send,
        T: Serialize + Debug + Send + Sync;

    /// sends PATCH request
    async fn patch<R, S, T>(
        &self,
        url: S,
        headers: Option<&Headers>,
        payload: T,
    ) -> Result<R, Error>
    where
        R: DeserializeOwned + Serialize + Debug,
        S: AsRef<str> + Sync + Send,
        T: Serialize + Debug + Send + Sync;

    /// sends DELETE request
    async fn delete<R, S, T>(
        &self,
        url: S,
        headers: Option<&Headers>,
        payload: T,
    ) -> Result<R, Error>
    where
        R: DeserializeOwned + Serialize + Debug,
        S: AsRef<str> + Sync + Send,
        T: Serialize + Debug + Send + Sync;
}
// ────────────────────────────────────────────────────────────
// vim: filetype=rust syntax=rust softtabstop=4 tabstop=4 shiftwidth=4 textwidth=79 fileencoding=utf-8 expandtab
// code: language=rust insertSpaces=true tabSize=4
