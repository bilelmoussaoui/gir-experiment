use xmlserde::Unparsed;
use xmlserde_derives::XmlDeserialize;

use crate::{attribute::Attribute, function::Function, prelude::*, version::Version, Stability};

#[derive(Debug, XmlDeserialize)]
pub struct Boxed {
    #[xmlserde(name = b"glib:type-name", ty = "attr")]
    g_type_name: Option<String>,
    #[xmlserde(name = b"glib:get-type", ty = "attr")]
    g_get_type: Option<String>,
    #[xmlserde(name = b"c:symbol-prefix", ty = "attr")]
    symbol_prefix: Option<String>,
    #[xmlserde(name = b"foreign", ty = "attr")]
    foreign: Option<bool>,
    #[xmlserde(name = b"glib:is-gtype-struct-for", ty = "attr")]
    g_is_gtype_struct_for: Option<String>,
    #[xmlserde(name = b"copy-function", ty = "attr")]
    copy_function: Option<String>,
    #[xmlserde(name = b"free-function", ty = "attr")]
    free_function: Option<String>,

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

    #[xmlserde(name = b"function", ty = "child")]
    functions: Vec<Function>,
}

impl Boxed {
    pub fn is_foreign(&self) -> bool {
        self.foreign.unwrap_or(false)
    }

    pub fn g_is_gtype_struct_for(&self) -> Option<&str> {
        self.g_is_gtype_struct_for.as_deref()
    }

    pub fn g_type_name(&self) -> Option<&str> {
        self.g_type_name.as_deref()
    }

    pub fn g_get_type(&self) -> Option<&str> {
        self.g_get_type.as_deref()
    }

    pub fn symbol_prefix(&self) -> Option<&str> {
        self.symbol_prefix.as_deref()
    }

    pub fn copy_function(&self) -> Option<&str> {
        self.copy_function.as_deref()
    }

    pub fn free_function(&self) -> Option<&str> {
        self.free_function.as_deref()
    }

    pub fn functions(&self) -> &[Function] {
        &self.functions
    }
}

impl_info!(Boxed);
impl_attributable!(Boxed);
impl_documentable!(Boxed);
