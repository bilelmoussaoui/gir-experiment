use serde::{Deserialize, Deserializer};

use crate::{r#type::Type, transfer::TransferOwnership, version::Version};

#[derive(Debug, Deserialize)]
pub struct FunctionReturn {
    #[serde(default, rename = "@transfer-ownership")]
    transfer: TransferOwnership,
    #[serde(default, rename = "type")]
    type_: Option<Type>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum FunctionScope {
    Call,
    Notified,
    Async,
    Forever,
}

#[derive(Debug, Deserialize)]
pub struct Parameter {
    #[serde(rename = "@name")]
    name: String,
    #[serde(default, rename = "@transfer-ownership")]
    transfer: Option<TransferOwnership>,
    #[serde(default, rename = "@nullable")]
    nullable: Option<bool>,
    #[serde(default, rename = "@allow-none")]
    allow_none: Option<bool>,
    #[serde(default, rename = "@scope")]
    scope: Option<FunctionScope>,
    #[serde(default, rename = "@closure")]
    closure: Option<u8>,
    #[serde(default, rename = "@destroy")]
    destroy: Option<u8>,
    #[serde(default)]
    doc: Option<String>,
    #[serde(default, rename = "type")]
    type_: Option<Type>,
}

pub(super) fn unwrap_parameters<'de, D>(deserializer: D) -> Result<Vec<Parameter>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Parameters {
        #[serde(default, rename = "instance-parameter")]
        instance_parameter: Option<Parameter>,
        #[serde(default)]
        parameter: Vec<Parameter>,
    }
    let mut params = Parameters::deserialize(deserializer)?;
    if let Some(instance_param) = params.instance_parameter {
        params.parameter.insert(0, instance_param);
    }
    Ok(params.parameter)
}

#[derive(Debug, Deserialize)]
pub struct Function {
    #[serde(rename = "@name")]
    name: String,
    #[serde(default, rename = "@identifier")]
    c_identifier: Option<String>,
    #[serde(default, rename = "@version")]
    version: Option<Version>,
    #[serde(default, rename = "@deprecated-version")]
    deprecated_version: Option<Version>,
    #[serde(default, rename = "@deprecated")]
    deprecated: bool,
    #[serde(default, rename = "@get-property")]
    get_property: Option<String>,
    #[serde(default, rename = "@set-property")]
    set_property: Option<String>,
    #[serde(rename = "return-value")]
    return_value: FunctionReturn,
    #[serde(default, deserialize_with = "unwrap_parameters")]
    parameters: Vec<Parameter>,
    #[serde(default)]
    doc: Option<String>,
    #[serde(default, rename = "doc-deprecated")]
    doc_deprecated: Option<String>,
}
