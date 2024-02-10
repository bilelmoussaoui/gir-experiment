#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed at parsing the GIR file")]
    Parser(#[from] parser::ParserError),
    #[error("Failed at parsing the Gir.toml config")]
    Config(#[from] config::ParserError),
    #[error("I/O")]
    IO(#[from] std::io::Error),
    #[error("GIR file {0} not found in the following directories {1:#?}")]
    GirFileNotFound(String, Vec<String>),
    #[error("Failed to render template")]
    Template(#[from] tera::Error),
}
