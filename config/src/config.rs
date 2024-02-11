use std::path::{Path, PathBuf};

use parser::version::Version;
use serde::Deserialize;

use super::ParserError;
use crate::{enums::Mode, object::Object, Concurrency};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Options {
    library: String,
    version: Version,
    min_cfg_version: Version,
    target_path: Option<PathBuf>,
    concurrency: Option<Concurrency>,
    generate_display_trait: Option<bool>,
    #[serde(default)]
    external_libraries: Vec<String>,
    #[serde(default)]
    girs_directories: Vec<PathBuf>,
    deprecate_by_min_version: Option<bool>,
    use_gi_docgen: Option<bool>,
    generate_safety_asserts: Option<bool>,
    single_version_file: Option<bool>,
    generate_builder: Option<bool>,
    disable_format: Option<bool>,
    trust_return_value_nullability: Option<bool>,
    #[serde(default)]
    generate: Vec<String>,
    #[serde(default)]
    ignore: Vec<String>,
    #[serde(default)]
    manual: Vec<String>,
    work_mode: Option<Mode>,
}

impl Options {
    pub fn package_file(&self) -> String {
        format!("{}-{}.gir", self.library, self.version)
    }

    pub fn girs_directories(&self) -> &[PathBuf] {
        &self.girs_directories
    }

    pub fn work_mode(&self) -> Option<Mode> {
        self.work_mode
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    options: Options,
    #[serde(default)]
    object: Vec<Object>,
    external_libraries: Option<toml::Value>,
}

impl Config {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, ParserError> {
        let buffer = std::fs::read_to_string(path)?;
        toml::from_str(&buffer).map_err(From::from)
    }

    pub fn options(&self) -> &Options {
        &self.options
    }
}
