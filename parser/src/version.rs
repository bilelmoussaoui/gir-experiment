use std::str::FromStr;

use xmlserde::XmlValue;

#[derive(serde::Deserialize, Debug, PartialEq)]
#[cfg_attr(test, derive(Default))]
pub struct Version(String);

impl XmlValue for Version {
    fn serialize(&self) -> String {
        self.0.clone()
    }

    fn deserialize(s: &str) -> Result<Self, String> {
        // TODO: validate versions here maybe?
        Ok(Self(s.to_owned()))
    }
}

impl Version {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl PartialEq<str> for Version {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl FromStr for Version {
    type Err = super::ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: validate
        Ok(Self(s.to_owned()))
    }
}
