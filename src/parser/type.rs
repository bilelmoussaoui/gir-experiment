use xmlserde_derives::XmlDeserialize;

#[derive(Debug, XmlDeserialize)]
pub struct Type {
    #[xmlserde(name = b"name", ty = "attr")]
    name: Option<String>,
    #[xmlserde(name = b"c:type", ty = "attr")]
    c_type: Option<String>,
}

impl Type {
    pub fn c_type(&self) -> Option<&str> {
        self.c_type.as_deref()
    }
}
