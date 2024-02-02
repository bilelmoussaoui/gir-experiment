use super::{namespace::Namespace, version::Version};
use xmlserde_derives::XmlDeserialize;

#[derive(Debug, XmlDeserialize)]
pub struct Include {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"version", ty = "attr")]
    version: Option<String>,
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
    version: String,
    #[xmlserde(name = b"include", ty = "child")]
    includes: Vec<Include>,
    #[xmlserde(name = b"c:include", ty = "child")]
    c_includes: Vec<Include>,
    #[xmlserde(name = b"package", ty = "child")]
    packages: Vec<Package>,
    #[xmlserde(name = b"namespace", ty = "child")]
    namespace: Namespace,
}

impl Repository {
    //pub fn namespace(&self) -> &Namespace {
    //    &self.namespace
    //}
}
