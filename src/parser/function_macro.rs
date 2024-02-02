use xmlserde_derives::XmlDeserialize;

use super::function::Parameters;
use crate::version::Version;

#[derive(Debug, XmlDeserialize)]
pub struct FunctionMacro {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"c:identifier", ty = "attr")]
    c_identifier: String,
    #[xmlserde(name = b"introspectable", ty = "attr")]
    introspectable: bool,
    #[xmlserde(name = b"version", ty = "attr")]
    version: Option<Version>,
    #[xmlserde(name = b"parameters", ty = "child")]
    parameters: Option<Parameters>,
}
