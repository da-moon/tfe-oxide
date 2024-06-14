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
// ────────────────────────────────────────────────────────────
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
/// This struct represents a successful JSON:API response.
#[derive(
    PartialEq, Debug, Clone, Default, Deserialize, Serialize, Getters,
)]
pub struct Success<ATTRIBUTES, RELATIONSHIPS> {
    data: Data<ATTRIBUTES, RELATIONSHIPS>,
    #[serde(skip_serializing_if = "Option::is_none")]
    included: Option<Vec<Data<ATTRIBUTES, RELATIONSHIPS>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    links: Option<Links>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<Meta>,
}
// ────────────────────────────────────────────────────────────
impl<ATTRIBUTES, RELATIONSHIPS> From<serde_json::Value>
    for Success<ATTRIBUTES, RELATIONSHIPS>
where
    ATTRIBUTES: DeserializeOwned + Serialize,
    RELATIONSHIPS: DeserializeOwned + Serialize,
{
    /// Convert a JSON Value into a Success object
    fn from(arg: serde_json::Value) -> Self {
        serde_json::from_value::<Success<ATTRIBUTES, RELATIONSHIPS>>(arg)
            .unwrap()
    }
}
impl<ATTRIBUTES, RELATIONSHIPS> From<Success<ATTRIBUTES, RELATIONSHIPS>>
    for serde_json::Value
where
    ATTRIBUTES: DeserializeOwned + Serialize,
    RELATIONSHIPS: DeserializeOwned + Serialize,
{
    /// Convert a Success object into a JSON Value
    fn from(arg: Success<ATTRIBUTES, RELATIONSHIPS>) -> Self {
        serde_json::to_value(arg).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use std::any::Any;

    // cargo test --all-targets -- "jsonapi::success::tests" --nocapture
    // cargo watch -cx 'test --all-targets -- "jsonapi::success::tests" --nocapture'
    use serde_json::json;
    use snapbox::{assert_eq, assert_eq_path};
    #[test]
    fn test_data_struct_deserialize() -> Result<(), serde_json::Error> {
        #[derive(PartialEq, Debug, Default, serde::Deserialize)]
        struct TestAttributes {
            key: String,
            value: String,
            category: String,
            hcl: bool,
            sensitive: bool,
        };
        // NOTE : example is taken from the following
        // https://developer.hashicorp.com/terraform/cloud-docs/api-docs
        let expected: crate::jsonapi::Data<TestAttributes, serde_json::Value> =
            crate::jsonapi::Data {
                id: None,
                _type: "vars".to_string(),
                attributes: Some(TestAttributes {
                    key: "some_key".to_string(),
                    value: "some_value".to_string(),
                    category: "terraform".to_string(),
                    hcl: false,
                    sensitive: false,
                }),
                relationships: Some(json!({
                    "workspace": {
                        "data": {
                            "id": "ws-4j8p6jX1w33MiDC7",
                            "type": "workspaces"
                        }
                    }
                })),
                ..Default::default()
            };
        let input = json!({
            "type": "vars",
            "attributes": {
                "key": "some_key",
                "value": "some_value",
                "category": "terraform",
                "hcl": false,
                "sensitive": false
            },
            "relationships": {
                "workspace": {
                    "data": {
                        "id": "ws-4j8p6jX1w33MiDC7",
                        "type": "workspaces",
                    }
                }
            }
        });
        // Attempt to deserialize the JSON string into MyStruct<String>
        let actual: crate::jsonapi::Data<TestAttributes, serde_json::Value> =
            serde_json::from_value(input)?;

        // let actual: Data<TestAttributes> = serde_json::from_value(input)?;
        // Assertions to verify the deserialized data is as expected
        assert_eq!(actual, expected);
        Ok(())
    }
}
// ────────────────────────────────────────────────────────────
// vim: filetype=rust syntax=rust softtabstop=4 tabstop=4 shiftwidth=4 textwidth=79 fileencoding=utf-8 expandtab
// code: language=rust insertSpaces=true tabSize=4
