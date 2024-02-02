use xmlserde_derives::XmlDeserialize;

use super::r#type::Type;

#[derive(Debug, XmlDeserialize)]
pub struct Array {
    #[xmlserde(name = b"c:type", ty = "attr")]
    c_identifier: Option<String>,
    #[xmlserde(name = b"zero-terminated", ty = "attr")]
    zero_terminated: Option<bool>,
    #[xmlserde(name = b"fixed-size", ty = "attr")]
    fixed_size: Option<u16>,
    #[xmlserde(name = b"type", ty = "child")]
    type_: Type,
}
