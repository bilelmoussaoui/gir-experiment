use parser::version::Version;
use serde::Deserialize;

use crate::enums::{SafetyAssertion, StringType, Visibility};

#[derive(Debug, Deserialize)]
pub struct Parameter {
    name: Option<String>,
    pattern: Option<String>,
    nullable: Option<bool>,
    r#move: Option<bool>,
    r#const: Option<bool>,
    length_of: Option<String>,
    string_type: Option<StringType>,
    r#unsafe: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct FunctionReturn {
    nullable: Option<bool>,
    bool_return_is_error: Option<String>,
    nullable_return_is_error: Option<String>,
    use_return_for_result: Option<bool>,
    string_type: Option<StringType>,
    r#type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Function {
    name: Option<String>,
    pattern: Option<String>,
    manual: Option<bool>,
    ignore: Option<bool>,
    version: Option<Version>,
    cfg_condition: Option<String>,
    doc_hidden: Option<bool>,
    disable_length_detect: Option<bool>,
    no_future: Option<bool>,
    rename: Option<String>,
    assertion: Option<SafetyAssertion>,
    visibility: Option<Visibility>,
    #[serde(default)]
    doc_ignore_parameters: Vec<String>,
    generate_doc: Option<bool>,
    doc_trait_name: Option<String>,
    #[serde(default)]
    parameter: Vec<Parameter>,
    r#return: Option<FunctionReturn>,
    bypass_auto_rename: Option<bool>,
    constructor: Option<bool>,
}
