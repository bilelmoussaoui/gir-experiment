use serde::Deserialize;

use crate::transfer::TransferOwnership;

#[derive(Debug, Deserialize)]
pub struct Property {
    #[serde(rename = "@name")]
    name: String,
    #[serde(default, rename = "@writable")]
    writable: Option<bool>,
    #[serde(default, rename = "@construct-only")]
    construct_only: Option<bool>,
    #[serde(rename = "@transfer-ownership")]
    transfer_ownership: TransferOwnership,
    #[serde(default, rename = "@getter")]
    getter: Option<String>,
    #[serde(default, rename = "@default-value")]
    default_value: Option<String>,
}
