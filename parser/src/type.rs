use xmlserde::Unparsed;
use xmlserde_derives::XmlDeserialize;

use crate::{array::Array, prelude::*};

#[derive(Debug, XmlDeserialize)]
pub struct Type {
    #[xmlserde(name = b"name", ty = "attr")]
    name: Option<String>,
    #[xmlserde(name = b"c:type", ty = "attr")]
    c_type: Option<String>,
    #[xmlserde(name = b"introspectable", ty = "attr")]
    introspectable: Option<bool>,
    // Documentation
    #[xmlserde(name = b"doc", ty = "child")]
    doc: Option<Unparsed>,
    #[xmlserde(name = b"doc-deprecated", ty = "child")]
    doc_deprecated: Option<Unparsed>,
    // TODO: Does Type really take also a AnyType child? as that would cause an infinite recursion
}

impl Type {
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn c_type(&self) -> Option<&str> {
        self.c_type.as_deref()
    }

    pub fn is_introspectable(&self) -> bool {
        self.introspectable.unwrap_or(true)
    }
}

impl_documentable!(Type);

#[derive(Debug, XmlDeserialize)]
pub enum AnyType {
    #[xmlserde(name = b"type")]
    Type(Type),
    #[xmlserde(name = b"array")]
    Array(Array),
}

impl AnyType {
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    pub fn as_array(&self) -> &Array {
        match self {
            Self::Array(array) => array,
            _ => unreachable!(),
        }
    }

    pub fn is_type(&self) -> bool {
        matches!(self, Self::Type(_))
    }

    pub fn as_type(&self) -> &Type {
        match self {
            Self::Type(ty) => ty,
            _ => unreachable!(),
        }
    }
}
