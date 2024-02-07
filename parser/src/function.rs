use xmlserde::Unparsed;
use xmlserde_derives::XmlDeserialize;

use crate::{
    attribute::Attribute, parameter::Parameters, prelude::*, return_value::ReturnValue,
    version::Version, Stability,
};

#[derive(Debug, XmlDeserialize)]
pub struct Function {
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
    #[xmlserde(name = b"parameters", ty = "child", default = "Parameters::default")]
    parameters: Parameters,
}

impl Function {
    pub fn return_value(&self) -> &ReturnValue {
        &self.return_value
    }

    pub fn parameters(&self) -> &Parameters {
        &self.parameters
    }
}

impl_info!(Function);
impl_attributable!(Function);
impl_documentable!(Function);
impl_callable!(Function);

#[derive(Debug, XmlDeserialize)]
pub struct FunctionInline {
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
    #[xmlserde(name = b"parameters", ty = "child", default = "Parameters::default")]
    parameters: Parameters,
}

impl FunctionInline {
    pub fn return_value(&self) -> &ReturnValue {
        &self.return_value
    }

    pub fn parameters(&self) -> &Parameters {
        &self.parameters
    }
}

impl_info!(FunctionInline);
impl_attributable!(FunctionInline);
impl_documentable!(FunctionInline);
impl_callable!(FunctionInline);
