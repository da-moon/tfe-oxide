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
impl Display for Error {
    /// helps with pretty printing the error as string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = self.get_status();
        let title = self.get_title().trim();
        let title = title.trim_start_matches('.').trim_end_matches('.');
        write!(f, "Error({})", status)?;
        if title.len() > 0 || self.get_detail().is_some() {
            write!(f, ":")?;
        }
        if title.len() > 0 {
            write!(f, " {}.", title)?;
        }
        if self.get_detail().is_some() {
            let detail: &String = self.get_detail().as_ref().unwrap();
            let detail = detail.trim_start_matches('.').trim_end_matches('.');
            write!(f, " {}.", detail)?;
        }
        Ok(())
    }
}

// ────────────────────────────────────────────────────────────
/// This Error type is what gets returned from Terraform Cloud APIs.
///
/// It seems like the response is always an array with
/// a single JSON API error object.
#[derive(
    PartialEq, Clone, Default, Deserialize, Serialize, Getters, Debug,
)]
#[getset(get = "pub with_prefix")]
pub(crate) struct Failure {
    pub(crate) errors: Vec<Error>,
}
// ────────────────────────────────────────────────────────────
impl Display for Failure {
    /// helps with pretty printing the server response as string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.get_errors().len() == 0 {
            return write!(f, "No errors.");
        }
        write!(f, "Failure: [")?;
        for (idx, elem) in (&self.errors).into_iter().enumerate() {
            write!(f, "{}", elem)?;
            if idx != self.errors.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    // cargo test --all-targets -- "jsonapi::error::tests" --nocapture
    // cargo watch -cx 'test --all-targets -- "jsonapi::error::tests" --nocapture'
    use snapbox::{assert_eq, assert_eq_path};
    #[test]
    fn test_single_error() {
        let input: crate::jsonapi::Error = crate::jsonapi::Error {
            title: "not found".to_string(),
            detail: None,
            status: "404".to_string(),
            ..Default::default()
        };
        let expected = "Error(404): not found.";
        let actual = input.to_string();
        assert_eq(expected, actual);
    }
    #[test]
    fn test_error_with_detail() {
        let input = crate::jsonapi::Error {
            title: "not found".to_string(),
            detail: Some(
                "The requested resource could not be found.".to_string(),
            ),
            status: "404".to_string(),
            ..Default::default()
        };
        let expected =
            "Error(404): not found. The requested resource could not be found.";
        let actual = input.to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_error_without_title() {
        let input = crate::jsonapi::Error {
            title: "".to_string(),
            detail: Some(
                "The requested resource could not be found.".to_string(),
            ),
            status: "404".to_string(),
            ..Default::default()
        };
        let expected =
            "Error(404): The requested resource could not be found.";
        let actual = input.to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_error_without_detail() {
        let input = crate::jsonapi::Error {
            title: "not found".to_string(),
            status: "404".to_string(),
            ..Default::default()
        };
        let expected = "Error(404): not found.";
        let actual = input.to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_error_without_title_and_detail() {
        let input = crate::jsonapi::Error {
            title: "".to_string(),
            status: "404".to_string(),
            ..Default::default()
        };
        let expected = "Error(404)";
        let actual = input.to_string();
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_error_with_source_pointer() {
        let input = crate::jsonapi::Error {
            title: "invalid attribute".to_string(),
            detail: Some("Name has already been taken".to_string()),
            status: "422".to_string(),
            source: Some(crate::jsonapi::ErrorSource {
                pointer: Some("/data/attributes/name".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let expected =
            "Error(422): invalid attribute. Name has already been taken.";
        let actual = input.to_string();
        assert_eq!(expected, actual);
    }
    // ────────────────────────────────────────────────────────────
}
// ────────────────────────────────────────────────────────────
// vim: filetype=rust syntax=rust softtabstop=4 tabstop=4 shiftwidth=4 textwidth=79 fileencoding=utf-8 expandtab
// code: language=rust insertSpaces=true tabSize=4
