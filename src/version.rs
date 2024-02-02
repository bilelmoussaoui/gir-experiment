use xmlserde::XmlValue;

#[derive(serde::Deserialize, Debug)]
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

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
