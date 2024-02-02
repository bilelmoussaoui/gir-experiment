use super::r#type::Type;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Alias {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@type")]
    c_type: String,
    #[serde(rename = "type")]
    type_: Type,
}
