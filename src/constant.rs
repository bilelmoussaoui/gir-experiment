use serde::Deserialize;

use crate::r#type::Type;

#[derive(Debug, Deserialize)]
pub struct Constant {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@value")]
    value: String,
    #[serde(rename = "@type")]
    c_identifier: String,
    #[serde(default)]
    doc: Option<String>,
    #[serde(rename = "type")]
    type_: Type,
}
