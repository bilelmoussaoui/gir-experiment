#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("I/O operation failed")]
    IO(#[from] std::io::Error),
    #[error("Failed to parse toml file: {0}")]
    Toml(#[from] toml::de::Error),
}

pub mod config;
