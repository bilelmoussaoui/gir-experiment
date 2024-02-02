use std::path::Path;

use xmlserde_derives::XmlDeserialize;

use super::{namespace::Namespace, ParserError};
use crate::version::Version;

#[derive(Debug, XmlDeserialize)]
pub struct NamespaceInclude {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"version", ty = "attr")]
    version: Version,
}

impl NamespaceInclude {
    pub fn as_package(&self) -> String {
        format!("{}-{}.gir", self.name, self.version)
    }

    pub fn as_package_file(&self) -> String {
        format!("{}-{}.gir", self.name, self.version)
    }
}

#[derive(Debug, XmlDeserialize)]
pub struct HeaderInclude {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
}

#[derive(Debug, XmlDeserialize)]
pub struct Package {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
}

#[derive(Debug, XmlDeserialize)]
#[xmlserde(root = b"repository")]
pub struct Repository {
    #[xmlserde(name = b"version", ty = "attr")]
    version: Version,
    #[xmlserde(name = b"include", ty = "child")]
    includes: Vec<NamespaceInclude>,
    #[xmlserde(name = b"c:include", ty = "child")]
    c_includes: Vec<HeaderInclude>,
    #[xmlserde(name = b"package", ty = "child")]
    packages: Vec<Package>,
    #[xmlserde(name = b"namespace", ty = "child")]
    namespace: Namespace,
}

impl Repository {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, ParserError> {
        tracing::info!("Parsing {}", path.as_ref().display());
        let content = std::fs::read_to_string(path)?;
        let repository = xmlserde::xml_deserialize_from_str(&content).map_err(ParserError::Xml)?;
        Ok(repository)
    }

    pub fn namespace_includes(&self) -> &[NamespaceInclude] {
        &self.includes
    }

    pub fn namespace(&self) -> &Namespace {
        &self.namespace
    }
}
