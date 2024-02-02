use serde::Deserialize;

use super::r#type::Type;

#[derive(Debug, Deserialize)]
pub struct Array {
    #[serde(default, rename = "@type")]
    c_identifier: Option<String>,
    #[serde(default, rename = "@zero-terminated")]
    zero_terminated: Option<bool>,
    #[serde(default, rename = "@fixed-size")]
    fixed_size: Option<u16>,
    #[serde(rename = "type")]
    type_: Type,
}
