use parser::version::Version;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Member {
    name: Option<String>,
    pattern: Option<String>,
    version: Option<Version>,
    ignore: Option<bool>,
}
