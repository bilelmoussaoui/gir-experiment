use xmlserde::Unparsed;
use xmlserde_derives::XmlDeserialize;

use crate::{
    attribute::Attribute, prelude::*, r#type::Type, version::Version, Stability, TransferOwnership,
};

#[derive(Debug, XmlDeserialize)]
pub struct Property {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"readable", ty = "attr")]
    readable: Option<bool>,
    #[xmlserde(name = b"writable", ty = "attr")]
    writable: Option<bool>,
    #[xmlserde(name = b"construct", ty = "attr")]
    construct: Option<bool>,
    #[xmlserde(name = b"construct-only", ty = "attr")]
    construct_only: Option<bool>,
    #[xmlserde(name = b"setter", ty = "attr")]
    setter: Option<String>,
    #[xmlserde(name = b"getter", ty = "attr")]
    getter: Option<String>,
    #[xmlserde(name = b"default-value", ty = "attr")]
    default_value: Option<String>,
    #[xmlserde(name = b"transfer-ownership", ty = "attr")]
    transfer_ownership: TransferOwnership,
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
    type_: Type, // TODO: should this really be a AnyType?
}

impl Property {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_readable(&self) -> bool {
        self.readable.unwrap_or(false)
    }

    pub fn is_writable(&self) -> bool {
        self.writable.unwrap_or(false)
    }

    pub fn is_construct(&self) -> bool {
        self.construct.unwrap_or(false)
    }

    pub fn is_construct_only(&self) -> bool {
        self.construct_only.unwrap_or(false)
    }

    pub fn transfer_ownership(&self) -> TransferOwnership {
        self.transfer_ownership
    }

    pub fn getter(&self) -> Option<&str> {
        self.getter.as_deref()
    }

    pub fn setter(&self) -> Option<&str> {
        self.setter.as_deref()
    }

    pub fn default_value(&self) -> Option<&str> {
        self.default_value.as_deref()
    }

    pub fn ty(&self) -> &Type {
        &self.type_
    }
}

impl_info!(Property);
impl_documentable!(Property);
impl_attributable!(Property);
