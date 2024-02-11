#![allow(unused)]

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("I/O operation failed")]
    IO(#[from] std::io::Error),
    #[error("Failed to parse toml file: {0}")]
    Toml(#[from] toml::de::Error),
}

mod config;
pub use config::Config;
mod enums;
pub use enums::{Concurrency, Mode, SafetyAssertion, StringType, Visibility};
mod function;
mod object;
pub use object::Object;
mod constant;
mod member;
mod property;
mod signal;
