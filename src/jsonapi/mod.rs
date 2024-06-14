//! This module implements JSON:API Request and Response objects.
//!
//! The JSON:API specification is available at [jsonapi.org](https://jsonapi.org/).

/// This module contains objects that make up a successful JSON:API response.
pub mod success;

pub type ListOptions = success::ListOptions;
pub type Pagination = success::Pagination;
pub type Meta = success::Meta;
pub type Links = success::Links;
// ────────────────────────────────────────────────────────────
// vim: filetype=rust syntax=rust softtabstop=4 tabstop=4 shiftwidth=4 textwidth=79 fileencoding=utf-8 expandtab
// code: language=rust insertSpaces=true tabSize=4
