use xmlserde_derives::XmlDeserialize;

use crate::enums::TransferOwnership;

#[derive(Debug, XmlDeserialize)]
pub struct Property {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"writable", ty = "attr")]
    writable: Option<bool>,
    #[xmlserde(name = b"construct-only", ty = "attr")]
    construct_only: Option<bool>,
    #[xmlserde(name = b"transfer-ownership", ty = "attr")]
    transfer_ownership: TransferOwnership,
    #[xmlserde(name = b"getter", ty = "attr")]
    getter: Option<String>,
    #[xmlserde(name = b"default-value", ty = "attr")]
    default_value: Option<String>,
}
