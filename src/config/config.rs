use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Options {
    library: String,
    version: String,
    min_cfg_version: String,
    target_path: PathBuf,
    #[serde(default)]
    girs_directories: Vec<PathBuf>,
    deprecate_by_min_version: Option<bool>,
    use_gi_docgen: Option<bool>,
    generate_safety_asserts: Option<bool>,
    single_version_file: Option<bool>,
    generate_builder: Option<bool>,
    disable_format: Option<bool>,
    trust_return_value_nullability: Option<bool>,
    generate: Vec<String>,
    ignore: Vec<String>,
    manual: Vec<String>,
}

impl Options {
    pub fn package_file(&self) -> String {
        format!("{}-{}.gir", self.library, self.version)
    }

    pub fn girs_directories(&self) -> &[PathBuf] {
        &self.girs_directories
    }
}

#[derive(Debug, Deserialize)]
pub struct Parameter {
    name: Option<String>,
    pattern: Option<String>,
    nullable: Option<bool>,
    r#move: Option<bool>,
    r#const: Option<bool>,
    length_of: Option<String>,
    string_type: Option<String>, // use enum
    r#unsafe: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct FunctionReturn {
    nullable: Option<bool>,
    bool_return_is_error: Option<String>,
    nullable_return_is_error: Option<String>,
    use_return_for_result: Option<bool>,
    string_type: Option<String>, // use enum
    r#type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Function {
    name: Option<String>,
    pattern: Option<String>,
    manual: Option<bool>,
    ignore: Option<bool>,
    version: Option<String>,
    cfg_condition: Option<String>,
    doc_hidden: Option<bool>,
    disable_length_detect: Option<bool>,
    no_future: Option<bool>,
    rename: Option<String>,
    assertion: Option<String>,  // Use enum
    visibility: Option<String>, // Use enum
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
    version: Option<String>,
    cfg_condition: Option<String>,
    trust_return_value_nullability: Option<bool>,
    visibility: Option<String>, // Use enum
    default_value: Option<String>,
    generate_doc: Option<bool>,
    #[serde(default)]
    manual_traits: Vec<String>,
    #[serde(default)]
    function: Vec<Function>,
    concurrency: Option<String>, // Use enum
}

#[derive(Debug, Deserialize)]
pub struct Config {
    options: Options,
    #[serde(default)]
    object: Vec<Object>,
    external_libraries: toml::Value,
}

impl Config {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, ()> {
        let buffer = std::fs::read_to_string(path).unwrap();
        Ok(toml::from_str(&buffer).unwrap())
    }

    pub fn options(&self) -> &Options {
        &self.options
    }
}