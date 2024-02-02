use crate::{namespace::Namespace, version::Version};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Include {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@version")]
    version: Version,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    #[serde(rename = "@name")]
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct Repository {
    #[serde(rename = "@version")]
    version: Version,
    //    #[serde(default, rename = "include")]
    //    includes: Vec<Include>,
    //    c_includes: Vec<String>,
    #[serde(default, rename = "package")]
    packages: Vec<Package>,
    namespace: Namespace,
}

impl Repository {
    pub fn namespace(&self) -> &Namespace {
        &self.namespace
    }
}
