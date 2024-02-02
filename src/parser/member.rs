use xmlserde::Unparsed;
use xmlserde_derives::XmlDeserialize;

#[derive(Debug, XmlDeserialize)]
pub struct Member {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"value", ty = "attr")]
    value: String,
    #[xmlserde(name = b"c:identifier", ty = "attr")]
    c_identifier: String,
    #[xmlserde(name = b"nick", ty = "attr")]
    nick: Option<String>,
    #[xmlserde(name = b"doc", ty = "child")]
    doc: Option<Unparsed>,
}

impl Member {
    pub fn c_identifier(&self) -> &str {
        &self.c_identifier
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
