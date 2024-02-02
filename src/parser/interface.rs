use xmlserde::Unparsed;
use xmlserde_derives::XmlDeserialize;

use super::{function::Function, property::Property, signal::Signal};

#[derive(Debug, XmlDeserialize)]
pub struct Prerequisite {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
}

#[derive(Debug, XmlDeserialize)]
pub struct Interface {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"c:symbol-prefix", ty = "attr")]
    symbol_prefix: String,
    #[xmlserde(name = b"c:type", ty = "attr")]
    c_type: Option<String>,
    #[xmlserde(name = b"glib:type-name", ty = "attr")]
    type_name: String,
    #[xmlserde(name = b"glib:get-type", ty = "attr")]
    get_type: String,
    #[xmlserde(name = b"glib:type-struct", ty = "attr")]
    type_struct: Option<String>,
    #[xmlserde(name = b"doc", ty = "child")]
    doc: Option<Unparsed>,
    #[xmlserde(name = b"prerequisite", ty = "child")]
    prerequisites: Vec<Prerequisite>,
    #[xmlserde(name = b"function", ty = "child")]
    functions: Vec<Function>,
    #[xmlserde(name = b"method", ty = "child")]
    methods: Vec<Function>,
    #[xmlserde(name = b"property", ty = "child")]
    properties: Vec<Property>,
    #[xmlserde(name = b"signal", ty = "child")]
    signals: Vec<Signal>,
    #[xmlserde(name = b"virtual-method", ty = "child")]
    virtual_methods: Vec<Function>,
}
