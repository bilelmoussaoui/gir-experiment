use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Union {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@type")]
    c_type: String,
}
