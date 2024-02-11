use parser::version::Version;
use serde::Deserialize;

use crate::{
    function::{FunctionReturn, Parameter},
    Concurrency,
};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Signal {
    name: Option<String>,
    pattern: Option<String>,
    manual: Option<bool>,
    ignore: Option<bool>,
    doc_trait_name: Option<String>,
    inhibit: Option<bool>,
    version: Option<Version>,
    #[serde(default)]
    parameter: Vec<Parameter>,
    r#return: Option<FunctionReturn>,
    concurrency: Option<Concurrency>,
}
