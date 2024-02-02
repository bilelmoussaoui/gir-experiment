use xmlserde_derives::XmlDeserialize;

use super::function::{FunctionReturn, Parameters};

#[derive(Debug, XmlDeserialize)]
pub struct Callback {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"c:type", ty = "attr")]
    c_type: Option<String>,
    #[xmlserde(name = b"return-value", ty = "child")]
    return_value: FunctionReturn,
    #[xmlserde(name = b"parameters", ty = "child")]
    parameters: Option<Parameters>,
}
