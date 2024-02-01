use serde::Deserialize;

use crate::r#type::Type;

#[derive(Debug, Deserialize)]
pub struct Array {
    #[serde(rename = "@zero-terminated")]
    zero_terminated: bool,
    #[serde(rename = "@fixed-size")]
    fixed_size: u16,
    #[serde(rename = "type")]
    type_: Type,
}
