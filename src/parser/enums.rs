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

#[derive(Debug, XmlDeserialize)]
pub struct Enumeration {
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
