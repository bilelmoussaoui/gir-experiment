use xmlserde::Unparsed;
use xmlserde_derives::XmlDeserialize;

use crate::{
    attribute::Attribute, parameter::Parameters, prelude::*, return_value::ReturnValue,
    version::Version, Stability,
};

#[derive(Debug, XmlDeserialize)]
pub struct VirtualMethod {
    #[xmlserde(name = b"invoker", ty = "attr")]
    invoker: Option<String>,
    // Callable attributes
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"c:identifier", ty = "attr")]
    c_identifier: Option<String>,
    #[xmlserde(name = b"shadows", ty = "attr")]
    shadows: Option<String>,
    #[xmlserde(name = b"shadowed-by", ty = "attr")]
    shadowed_by: Option<String>,
    #[xmlserde(name = b"throws", ty = "attr")]
    throws: Option<bool>,
    #[xmlserde(name = b"moved-to", ty = "attr")]
    moved_to: Option<String>,
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
    #[xmlserde(name = b"parameters", ty = "child")]
    parameters: Parameters,
}

impl VirtualMethod {
    pub fn invoker(&self) -> Option<&str> {
        self.invoker.as_deref()
    }

    pub fn return_value(&self) -> &ReturnValue {
        &self.return_value
    }

    pub fn parameters(&self) -> &Parameters {
        &self.parameters
    }
}

impl_info!(VirtualMethod);
impl_attributable!(VirtualMethod);
impl_documentable!(VirtualMethod);
impl_callable!(VirtualMethod);
