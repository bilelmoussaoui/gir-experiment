use xmlserde::{xml_serde_enum, Unparsed};
use xmlserde_derives::XmlDeserialize;

use super::{r#type::Type, transfer::TransferOwnership};

#[derive(Debug, XmlDeserialize)]
pub struct FunctionReturn {
    #[xmlserde(name = b"transfer-ownership", ty = "attr")]
    transfer: Option<TransferOwnership>,
    #[xmlserde(name = b"type", ty = "child")]
    type_: Option<Type>,
}

xml_serde_enum! {
    #[derive(Debug, Clone)]
    FunctionScope{
        Call => "call",
        Notified => "notified",
        Async => "async",
        Forever => "forever",
    }
}

#[derive(Debug, XmlDeserialize)]
pub struct Parameters {
    #[xmlserde(name = b"instance-parameter", ty = "child")]
    instance_parameter: Option<Parameter>,
    #[xmlserde(name = b"parameter", ty = "child")]
    parameter: Vec<Parameter>,
}

#[derive(Debug, XmlDeserialize)]
pub struct Parameter {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"transfer-ownership", ty = "attr")]
    transfer: Option<TransferOwnership>,
    #[xmlserde(name = b"nullable", ty = "attr")]
    nullable: Option<bool>,
    #[xmlserde(name = b"allow-none", ty = "attr")]
    allow_none: Option<bool>,
    #[xmlserde(name = b"scope", ty = "attr")]
    scope: Option<FunctionScope>,
    #[xmlserde(name = b"closure", ty = "attr")]
    closure: Option<u8>,
    #[xmlserde(name = b"destroy", ty = "attr")]
    destroy: Option<u8>,
    #[xmlserde(name = b"doc", ty = "child")]
    doc: Option<Unparsed>,
    #[xmlserde(name = b"type", ty = "child")]
    type_: Option<Type>,
}

#[derive(Debug, XmlDeserialize)]
pub struct Function {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"identifier", ty = "attr")]
    c_identifier: Option<String>,
    #[xmlserde(name = b"version", ty = "attr")]
    version: Option<String>,
    #[xmlserde(name = b"deprecated-version", ty = "attr")]
    deprecated_version: Option<String>,
    #[xmlserde(name = b"deprecated", ty = "attr")]
    deprecated: Option<bool>,
    #[xmlserde(name = b"get-property", ty = "attr")]
    get_property: Option<String>,
    #[xmlserde(name = b"set-property", ty = "attr")]
    set_property: Option<String>,
    #[xmlserde(name = b"introspectable", ty = "attr")]
    introspectable: Option<bool>,
    #[xmlserde(name = b"return-value", ty = "child")]
    return_value: FunctionReturn,
    #[xmlserde(name = b"parameters", ty = "child")]
    parameters: Option<Parameters>,
    #[xmlserde(name = b"doc", ty = "child")]
    doc: Option<Unparsed>,
    #[xmlserde(name = b"doc-deprecated", ty = "child")]
    doc_deprecated: Option<Unparsed>,
}
