use xmlserde_derives::XmlDeserialize;

use super::function::Parameters;

#[derive(Debug, XmlDeserialize)]
pub struct FunctionMacro {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"c:identifier", ty = "attr")]
    c_identifier: String,
    #[xmlserde(name = b"introspectable", ty = "attr")]
    introspectable: bool,
    #[xmlserde(name = b"version", ty = "attr")]
    version: Option<String>,
    #[xmlserde(name = b"parameters", ty = "child")]
    parameters: Option<Parameters>,
}
