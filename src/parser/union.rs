use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Union {
    #[serde(rename = "@name")]
    name: String,
    #[serde(default, rename = "@type")]
    c_type: Option<String>,
}
