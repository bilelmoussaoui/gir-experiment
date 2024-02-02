use serde::Deserialize;

#[derive(Default, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TransferOwnership {
    #[default]
    None,
    Container,
    Full,
}
