use parser::version::Version;
use serde::Deserialize;

use crate::{
    constant::Constant,
    enums::{Concurrency, ConversionType, RefMode, Visibility},
    function::Function,
    member::Member,
    property::Property,
    signal::Signal,
};

#[derive(Debug, Deserialize)]
pub struct Derive {
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Object {
    name: Option<String>,
    pattern: Option<String>,
    status: Option<String>,
    trait_name: Option<String>,
    ref_mode: Option<RefMode>,
    #[serde(default)]
    derive: Vec<Derive>,
    must_use: Option<bool>,
    generate_display_trait: Option<bool>,
    conversion_type: Option<ConversionType>,
    generate_builder: Option<bool>,
    builder_postprocess: Option<String>,
    final_type: Option<bool>,
    fundamental_type: Option<bool>,
    exhaustive: Option<bool>,
    boxed_inline: Option<bool>,
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
    #[serde(default)]
    signal: Vec<Signal>,
    #[serde(default)]
    member: Vec<Member>,
    #[serde(default)]
    property: Vec<Property>,
    #[serde(default)]
    constant: Vec<Constant>,
    concurrency: Option<Concurrency>,
}

impl Object {
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}
