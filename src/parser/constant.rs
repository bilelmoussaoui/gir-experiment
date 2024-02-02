use super::r#type::Type;
use xmlserde_derives::XmlDeserialize;

#[derive(Debug, XmlDeserialize)]
pub struct Constant {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"value", ty = "attr")]
    value: String,
    #[xmlserde(name = b"c:type", ty = "attr")]
    c_type: String,
    //#[xmlserde(name = b"doc", ty = "child")]
    //doc: Option<String>,
    #[xmlserde(name = b"type", ty = "child")]
    type_: Type,
}
