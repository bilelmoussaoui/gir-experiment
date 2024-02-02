use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Type {
    #[serde(rename = "@name")]
    name: String,
    #[serde(default, rename = "@type")]
    c_type: Option<String>,
}
