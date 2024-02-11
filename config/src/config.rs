use std::path::{Path, PathBuf};

use parser::version::Version;
use serde::{de, de::Visitor, Deserialize};

use super::ParserError;
use crate::{enums::Mode, object::Object, Concurrency};

#[derive(Debug)]
enum SingleVersion {
    Bool(bool),
    Path(PathBuf),
}

struct SingleVersionVisitor;

impl<'de> Visitor<'de> for SingleVersionVisitor {
    type Value = SingleVersion;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string or a boolean")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(SingleVersion::Bool(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(SingleVersion::Path(PathBuf::from(v)))
    }
}

impl<'de> Deserialize<'de> for SingleVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(SingleVersionVisitor)
    }
}

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
    girs_dir: Option<PathBuf>,
    #[serde(default)]
    girs_directories: Vec<PathBuf>,
    auto_path: Option<PathBuf>,
    deprecate_by_min_version: Option<bool>,
    use_gi_docgen: Option<bool>,
    generate_safety_asserts: Option<bool>,
    // This is so weird, why can it be both a boolean and a path???
    single_version_file: Option<SingleVersion>,
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

    pub fn girs_directories(&self) -> Vec<PathBuf> {
        let mut dirs = self.girs_directories.clone();
        if let Some(p) = &self.girs_dir {
            dirs.push(p.clone());
        }
        dirs
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
