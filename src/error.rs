#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed at parsing the GIR file")]
    Parser(#[from] crate::parser::ParserError),
    #[error("Failed at parsing the Gir.toml config")]
    Config(#[from] crate::config::ParserError),
    #[error("I/O")]
    IO(#[from] std::io::Error),
    #[error("GIR file {0} not found in the following directories {1:#?}")]
    GirFileNotFound(String, Vec<String>),
}
