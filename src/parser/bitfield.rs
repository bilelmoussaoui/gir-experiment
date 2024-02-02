use xmlserde::Unparsed;
use xmlserde_derives::XmlDeserialize;

use super::member::Member;

#[derive(Debug, XmlDeserialize)]
pub struct BitField {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"c:type", ty = "attr")]
    c_type: String,
    #[xmlserde(name = b"type-name", ty = "attr")]
    type_name: Option<String>,
    #[xmlserde(name = b"get-type", ty = "attr")]
    get_type: Option<String>,
    #[xmlserde(name = b"doc", ty = "child")]
    doc: Option<Unparsed>,
    #[xmlserde(name = b"member", ty = "child")]
    members: Vec<Member>,
}

impl BitField {
    pub fn c_type(&self) -> &str {
        &self.c_type
    }

    pub fn members(&self) -> &[Member] {
        &self.members
    }
}
