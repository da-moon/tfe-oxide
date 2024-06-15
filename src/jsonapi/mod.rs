//! This module implements JSON:API Request and Response objects.
//!
//! The JSON:API specification is available at [jsonapi.org](https://jsonapi.org/).

/// This module contains objects that make up a failed JSON:API response.
pub mod failure;
/// This module contains objects that make up a successful JSON:API response.
pub mod success;
// ────────────────────────────────────────────────────────────
pub type ListOptions = success::ListOptions;
pub type Pagination = success::Pagination;
// ────────────────────────────────────────────────────────────
pub type Meta = success::Meta;
pub type Links = success::Links;
pub type Data<ATTRIBUTES, RELATIONSHIPS> =
    success::Data<ATTRIBUTES, RELATIONSHIPS>;
pub type Success<ATTRIBUTES, RELATIONSHIPS> =
    success::Success<ATTRIBUTES, RELATIONSHIPS>;
// ────────────────────────────────────────────────────────────
pub type ErrorLinks = failure::Links;
pub type ErrorSource = failure::Source;
pub type Error = failure::Error;
// ────────────────────────────────────────────────────────────
// vim: filetype=rust syntax=rust softtabstop=4 tabstop=4 shiftwidth=4 textwidth=79 fileencoding=utf-8 expandtab
// code: language=rust insertSpaces=true tabSize=4
