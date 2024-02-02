use xmlserde_derives::XmlDeserialize;

#[derive(Debug, XmlDeserialize)]
pub struct Version {
    #[xmlserde(ty = "text")]
    inner: String,
}
