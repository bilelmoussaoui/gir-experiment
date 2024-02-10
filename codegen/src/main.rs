use std::{collections::HashMap, path::PathBuf};

use clap::Parser;
use cli::Args;
use config::Config;
use parser::repository::Repository;

mod cli;
mod codegen;
mod error;

pub type Result<T> = std::result::Result<T, error::Error>;

use config::Mode;

pub struct Library {
    repository: Repository,
    _sub_repositories: HashMap<String, Repository>,
    config: Config,
    args: Args,
}

fn find_file_or_fail(filename: &str, paths: &[PathBuf]) -> Result<PathBuf> {
    tracing::debug!("Trying to find gir file {filename} in {:#?}", paths);
    for parent in paths {
        let parent = match parent.canonicalize() {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                tracing::warn!(
                    "GIR directory {} could not be found, skipping",
                    parent.display()
                );
                continue;
            }
            p => p,
        }?;

        let gir_file = parent.canonicalize()?.join(filename);
        if gir_file.exists() {
            tracing::info!("GIR file found at {}", gir_file.display());
            return Ok(gir_file);
        }
    }
    Err(error::Error::GirFileNotFound(
        filename.to_owned(),
        paths
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>(),
    ))
}

impl Library {
    pub fn new(args: Args) -> Result<Self> {
        let config = Config::from_path(args.config())?;
        let mut girs_directories = config.options().girs_directories().to_owned();
        girs_directories.extend(args.girs_directories().to_owned());

        let package = &config.options().package_file();
        let main_namespace = Repository::from_path(find_file_or_fail(package, &girs_directories)?)?;

        let mut sub_repositories = HashMap::new();
        for include in main_namespace.namespace_includes() {
            if sub_repositories.contains_key(&include.as_package()) {
                continue;
            }

            let namespace = Repository::from_path(find_file_or_fail(
                &include.as_package_file(),
                &girs_directories,
            )?)?;
            sub_repositories.insert(include.as_package(), namespace);
        }

        Ok(Self {
            repository: main_namespace,
            _sub_repositories: sub_repositories,
            args,
            config,
        })
    }

    pub fn generate(&self) -> Result<()> {
        let mode: Mode = self
            .args
            .mode()
            .or(self.config.options().work_mode())
            .unwrap_or_default();
        match mode {
            Mode::Doc => todo!(),
            Mode::Normal => {
                codegen::auto::generate(self, "tests")?;
            }
            Mode::Sys => {
                codegen::sys::generate(self, "tests/sys")?;
            }
            Mode::NotBound => todo!(),
        };
        Ok(())
    }
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    let library = Library::new(args)?;
    library.generate()?;
    Ok(())
}
