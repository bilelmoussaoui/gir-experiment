use super::r#type::Type;
use xmlserde_derives::XmlDeserialize;

#[derive(Debug, XmlDeserialize)]
pub struct Alias {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"c:type", ty = "attr")]
    c_type: String,
    #[xmlserde(name = b"type", ty = "child")]
    type_: Type,
}

impl Alias {
    pub fn c_type(&self) -> &str {
        &self.c_type
    }

    pub fn ty(&self) -> &Type {
        &self.type_
    }
}
