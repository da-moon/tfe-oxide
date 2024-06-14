use {
    getset::{CopyGetters, Getters},
    serde::{de::DeserializeOwned, Deserialize, Serialize},
    std::{
        fmt::{Debug, Display, Write},
        rc::Rc,
    },
};
/// This struct contains fields used in Query Parameters for passing in
/// pagination options
#[derive(
    PartialEq, Debug, Clone, Default, Deserialize, Serialize, Getters,
)]
#[getset(get = "pub with_prefix")]
#[serde(rename_all = "kebab-case")]
pub struct ListOptions {
    current_page: u32,
    total_pages: u32,
    total_count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    prev_page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_page: Option<u32>,
}
/// This struct contains fields that are part of server response for endpoints
/// that return a list of resources which support pagination.
#[derive(
    PartialEq, Debug, Clone, Default, Deserialize, Serialize, Getters,
)]
#[getset(get = "pub with_prefix")]
#[serde(rename_all = "kebab-case")]
pub struct Pagination {
    current_page: u32,
    total_pages: u32,
    total_count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    prev_page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_page: Option<u32>,
}
/// This struct contains represents object that contains non-standard
/// meta-information in a response
#[derive(
    PartialEq, Debug, Clone, Default, Deserialize, Serialize, Getters,
)]
#[getset(get = "pub with_prefix")]
pub struct Meta {
    // XXX: is pagination optional ?
    #[serde(skip_serializing_if = "Option::is_none")]
    pagination: Option<Pagination>,
}
// ────────────────────────────────────────────────────────────
/// This struct contains represents object that contains links relevant API resources
#[derive(
    PartialEq, Debug, Clone, Default, Deserialize, Serialize, Getters,
)]
#[getset(get = "pub with_prefix")]
pub struct Links {
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    _self: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    related: Option<String>,
}
/// This struct contains the actual data of the response. In most cases, you
/// want to only extract the data field from the response.
#[derive(
    PartialEq, Debug, Clone, Default, Deserialize, Serialize, Getters,
)]
#[getset(get = "pub with_prefix")]
pub struct Data<ATTRIBUTES, RELATIONSHIPS> {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<ATTRIBUTES>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<RELATIONSHIPS>,
}
// ────────────────────────────────────────────────────────────
