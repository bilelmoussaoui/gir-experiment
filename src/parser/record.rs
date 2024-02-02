use xmlserde::Unparsed;
use xmlserde_derives::XmlDeserialize;

use super::{array::Array, callback::Callback, function::Function, r#type::Type};

#[derive(Debug, XmlDeserialize)]
pub enum InnerField {
    #[xmlserde(name = b"type")]
    Type(Type),
    #[xmlserde(name = b"callback")]
    Callback(Callback),
    #[xmlserde(name = b"array")]
    Array(Array),
}

#[derive(Debug, XmlDeserialize)]
pub struct Field {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"readable", ty = "attr")]
    readable: Option<bool>,
    #[xmlserde(name = b"writable", ty = "attr")]
    writable: Option<bool>,
    #[xmlserde(name = b"private", ty = "attr")]
    private: Option<bool>,
    #[xmlserde(name = b"doc", ty = "child")]
    doc: Option<Unparsed>,
    #[xmlserde(ty = "untag")]
    inner: InnerField,
}

#[derive(Debug, XmlDeserialize)]
pub struct Record {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"c:type", ty = "attr")]
    c_type: Option<String>,
    #[xmlserde(name = b"disguised", ty = "attr")]
    disguised: Option<bool>,
    #[xmlserde(name = b"opaque", ty = "attr")]
    opaque: Option<bool>,
    #[xmlserde(name = b"version", ty = "attr")]
    version: Option<String>,
    #[xmlserde(name = b"deprecated", ty = "attr")]
    deprecated: Option<bool>,
    #[xmlserde(name = b"deprecated-version", ty = "attr")]
    deprecated_version: Option<String>,
    #[xmlserde(name = b"glib:is-gtype-struct-for", ty = "attr")]
    is_gtype_struct_for: Option<String>,
    #[xmlserde(name = b"glib:get-type", ty = "attr")]
    get_type: Option<String>,
    #[xmlserde(name = b"c:symbol-prefix", ty = "attr")]
    symbol_prefix: Option<String>,
    #[xmlserde(name = b"doc", ty = "child")]
    doc: Option<Unparsed>,
    #[xmlserde(name = b"field", ty = "child")]
    fields: Vec<Field>,
    #[xmlserde(name = b"function", ty = "child")]
    functions: Vec<Function>,
    #[xmlserde(name = b"method", ty = "child")]
    methods: Vec<Function>,
    #[xmlserde(name = b"constructor", ty = "child")]
    constructors: Vec<Function>,
}

impl Record {}
