use xmlserde::{xml_serde_enum, Unparsed};
use xmlserde_derives::XmlDeserialize;

use super::function::{FunctionReturn, Parameters};
use crate::{enums::SignalEmission, version::Version};

#[derive(Debug, XmlDeserialize)]
pub struct Signal {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"when", ty = "attr")]
    when: Option<SignalEmission>,
    #[xmlserde(name = b"version", ty = "attr")]
    version: Option<Version>,
    #[xmlserde(name = b"doc", ty = "child")]
    doc: Option<Unparsed>,
    #[xmlserde(name = b"return-value", ty = "child")]
    return_value: FunctionReturn,
    #[xmlserde(name = b"parameters", ty = "child")]
    parameters: Parameters,
}
