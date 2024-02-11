use clap::ValueEnum;
use serde::{
    de::{self, Visitor},
    Deserialize,
};

#[derive(ValueEnum, Deserialize, Default, Debug, Copy, Clone)]
#[clap(rename_all = "snake_case")]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Doc,
    Normal,
    Sys,
    #[default]
    NotBound,
}

#[derive(Deserialize, Default, Debug, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    #[default]
    Pub,
    Crate,
    Private,
    Super,
}

#[derive(Deserialize, Default, Debug, Copy, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum SafetyAssertion {
    #[default]
    None,
    Skip,
    NotInitialized,
    InMainThread,
    Super,
}

#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum StringType {
    Utf8,
    Filename,
    OsString,
}

#[derive(Deserialize, Default, Debug, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Concurrency {
    #[default]
    None,
    Send,
    #[serde(alias = "send+sync")]
    SendSync,
}

#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RefMode {
    Ref,
    #[serde(alias = "ref-mut")]
    RefMut,
}

#[derive(Debug)]
pub enum ConversionType {
    Scalar,
    Option,
    Custom {
        variant: String,
        ok_type: String,
        err_type: String,
    },
}

struct ConversionTypeVisitor;

impl<'de> Visitor<'de> for ConversionTypeVisitor {
    type Value = ConversionType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string or a map")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match v {
            "Option" => Ok(ConversionType::Option),
            "scalar" => Ok(ConversionType::Scalar),
            e => Err(E::custom("Unsupported custom conversion {e}")),
        }
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut variant = None;
        let mut ok_type = None;
        let mut err_type = None;
        while let Some((key, value)) = map.next_entry::<String, String>()? {
            match key.as_str() {
                "variant" => {
                    variant = Some(value);
                }
                "ok_type" => {
                    ok_type = Some(value);
                }
                "err_type" => {
                    err_type = Some(value);
                }
                e => {
                    return Err(de::Error::custom(
                        "Unsupported custom conversion attribute {e}",
                    ))
                }
            }
        }
        Ok(ConversionType::Custom {
            variant: variant.ok_or(de::Error::custom("conversion_type requires a variant"))?,
            ok_type: ok_type.ok_or(de::Error::custom("conversion_type requires a ok_type"))?,
            err_type: err_type.ok_or(de::Error::custom("conversion_type requires a err_type"))?,
        })
    }
}

impl<'de> Deserialize<'de> for ConversionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ConversionTypeVisitor)
    }
}
