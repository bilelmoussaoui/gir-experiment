use parser::version::Version;
use serde::Deserialize;

use crate::{
    enums::{Concurrency, Visibility},
    function::Function,
};

#[derive(Debug, Deserialize)]
pub struct Object {
    name: Option<String>,
    pattern: Option<String>,
    status: Option<String>,
    trait_name: Option<String>,
    generate_builder: Option<bool>,
    builder_postprocess: Option<String>,
    final_type: Option<bool>,
    fundamental_type: Option<bool>,
    exhaustive: Option<bool>,
    module_name: Option<String>,
    version: Option<Version>,
    cfg_condition: Option<String>,
    trust_return_value_nullability: Option<bool>,
    visibility: Option<Visibility>,
    default_value: Option<String>,
    generate_doc: Option<bool>,
    #[serde(default)]
    manual_traits: Vec<String>,
    #[serde(default)]
    function: Vec<Function>,
    concurrency: Option<Concurrency>,
}
