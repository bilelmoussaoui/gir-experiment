use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Type {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@type")]
    c_type: String,
}
