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
pub(crate) struct ListOptions {
    current_page: u32,
    total_pages: u32,
    total_count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    prev_page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_page: Option<u32>,
}
