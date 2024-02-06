use xmlserde::Unparsed;
use xmlserde_derives::XmlDeserialize;

use crate::{attribute::Attribute, prelude::*, r#type::AnyType, version::Version, Stability};

#[derive(Debug, XmlDeserialize)]
pub struct Constant {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"value", ty = "attr")]
    value: String,
    #[xmlserde(name = b"c:type", ty = "attr")]
    c_type: String,
    #[xmlserde(name = b"c:identifier", ty = "attr")]
    c_identifier: Option<String>,
    // Common attributes
    #[xmlserde(name = b"introspectable", ty = "attr")]
    introspectable: Option<bool>,
    #[xmlserde(name = b"deprecated", ty = "attr")]
    deprecated: Option<bool>,
    #[xmlserde(name = b"version", ty = "attr")]
    version: Option<Version>,
    #[xmlserde(name = b"deprecated-version", ty = "attr")]
    deprecated_version: Option<Version>,
    #[xmlserde(name = b"stability", ty = "attr")]
    stability: Option<Stability>,
    // Documentation
    #[xmlserde(name = b"doc", ty = "child")]
    doc: Option<Unparsed>,
    #[xmlserde(name = b"doc-deprecated", ty = "child")]
    doc_deprecated: Option<Unparsed>,
    // Attributes: 0 or more
    #[xmlserde(name = b"attribute", ty = "child")]
    attributes: Vec<Attribute>,
    #[xmlserde(name = b"type", ty = "child")]
    type_: AnyType,
}

impl Constant {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn c_type(&self) -> &str {
        &self.c_type
    }

    pub fn c_identifier(&self) -> Option<&str> {
        self.c_identifier.as_deref()
    }

    pub fn ty(&self) -> &AnyType {
        &self.type_
    }
}

impl_info!(Constant);
impl_attributable!(Constant);
impl_documentable!(Constant);
