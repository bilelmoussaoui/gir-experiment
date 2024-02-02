use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Type {
    #[serde(default, rename = "@name")]
    name: Option<String>,
    #[serde(default, rename = "@type")]
    c_type: Option<String>,
}
