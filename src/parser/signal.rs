use xmlserde::xml_serde_enum;
use xmlserde::Unparsed;
use xmlserde_derives::XmlDeserialize;

use super::{
    function::{FunctionReturn, Parameters},
    version::Version,
};

xml_serde_enum! {
    #[derive(Debug, Clone)]
    Emission{
        First => "first",
        Last => "last",
        Cleanup => "cleanup",
    }
}

#[derive(Debug, XmlDeserialize)]
pub struct Signal {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"when", ty = "attr")]
    when: Option<Emission>,
    #[xmlserde(name = b"version", ty = "attr")]
    version: Option<String>,
    #[xmlserde(name = b"doc", ty = "child")]
    doc: Option<Unparsed>,
    #[xmlserde(name = b"return-value", ty = "child")]
    return_value: FunctionReturn,
    #[xmlserde(name = b"parameters", ty = "child")]
    parameters: Parameters,
}
