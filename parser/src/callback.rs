use xmlserde::Unparsed;
use xmlserde_derives::XmlDeserialize;

use crate::{
    attribute::Attribute, parameter::Parameters, prelude::*, return_value::ReturnValue,
    version::Version, Stability,
};

#[derive(Debug, XmlDeserialize)]
pub struct Callback {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"c:type", ty = "attr")]
    c_type: Option<String>,
    #[xmlserde(name = b"throws", ty = "attr")]
    throws: Option<bool>,
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
    #[xmlserde(name = b"return-value", ty = "child")]
    return_value: ReturnValue,
    #[xmlserde(name = b"parameters", ty = "child", default = "Parameters::default")]
    parameters: Parameters,
}

impl Callback {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn c_type(&self) -> Option<&str> {
        self.c_type.as_deref()
    }

    pub fn throws(&self) -> bool {
        self.throws.unwrap_or(false)
    }

    pub fn return_value(&self) -> &ReturnValue {
        &self.return_value
    }

    pub fn parameters(&self) -> &Parameters {
        &self.parameters
    }
}

impl_info!(Callback);
impl_attributable!(Callback);
impl_documentable!(Callback);
