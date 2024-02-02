use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Member {
    //#[serde(rename = "@name")]
    //name: String,
    #[serde(rename = "@value")]
    value: u32,
    #[serde(rename = "@identifier")]
    c_identifier: String,
    #[serde(rename = "@nick")]
    nick: String,
    #[serde(default)]
    doc: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Enumeration {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@type")]
    c_type: String,
    #[serde(rename = "@type-name")]
    type_name: String,
    #[serde(rename = "@get-type")]
    get_type: String,
    #[serde(default)]
    doc: Option<String>,
    #[serde(default, rename = "member")]
    members: Vec<Member>,
}
