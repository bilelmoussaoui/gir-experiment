use parser::version::Version;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PropertyFunc {
    Get,
    Notify,
    Set,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Property {
    name: Option<String>,
    pattern: Option<String>,
    #[serde(default)]
    generate: Vec<PropertyFunc>,
    version: Option<Version>,
    ignore: Option<bool>,
    manual: Option<bool>,
}
