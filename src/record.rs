use serde::Deserialize;

use crate::{array::Array, callback::Callback, r#type::Type, version::Version};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InnerField {
    Type(Type),
    Callback(Callback),
    Array(Array),
}

#[derive(Debug, Deserialize)]
pub struct Field {
    #[serde(rename = "@name")]
    name: String,
    #[serde(default, rename = "@readable")]
    readable: Option<bool>,
    #[serde(default, rename = "@writable")]
    writable: Option<bool>,
    #[serde(default, rename = "@private")]
    private: Option<bool>,
    #[serde(default)]
    doc: Option<String>,
    #[serde(rename = "$value")]
    inner: InnerField,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@type")]
    c_type: String,
    #[serde(default, rename = "@disguised")]
    disguised: bool,
    #[serde(default, rename = "@version")]
    version: Option<Version>,
    #[serde(default, rename = "@is-gtype-struct-for")]
    is_gtype_struct_for: Option<String>,
    #[serde(default)]
    doc: Option<String>,
    #[serde(default, rename = "field")]
    fields: Vec<Field>,
}
