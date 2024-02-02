use xmlserde::Unparsed;
use xmlserde_derives::XmlDeserialize;

use super::r#type::Type;

#[derive(Debug, XmlDeserialize)]
pub struct Constant {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"value", ty = "attr")]
    value: String,
    #[xmlserde(name = b"c:type", ty = "attr")]
    c_type: String,
    #[xmlserde(name = b"doc", ty = "child")]
    doc: Option<Unparsed>,
    #[xmlserde(name = b"type", ty = "child")]
    type_: Type,
}

impl Constant {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn c_type(&self) -> &str {
        &self.c_type
    }
}
