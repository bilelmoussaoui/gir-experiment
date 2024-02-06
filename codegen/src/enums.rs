use clap::ValueEnum;
use serde::Deserialize;

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
