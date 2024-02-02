use std::{collections::HashMap, path::PathBuf};

use clap::Parser;
use cli::Args;
use config::config::Config;

use crate::parser::repository::Repository;

mod cli;
mod codegen;
mod config;
mod parser;

pub struct Library {
    repository: Repository,
    namespaces: HashMap<String, Repository>,
    config: Config,
    args: Args,
}

fn find_file_or_fail(filename: &str, paths: &[PathBuf]) -> Result<PathBuf, String> {
    for parent in paths {
        let gir_file = parent.canonicalize().unwrap().join(filename);
        if gir_file.exists() {
            return Ok(gir_file);
        }
    }
    Err(format!(
        "File {filename} couldn't be found in the following directories: {}",
        paths
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join(",")
    ))
}

impl Library {
    pub fn new(args: Args) -> Result<Self, &'static str> {
        let config = Config::from_path(args.config()).unwrap();
        let mut girs_directories = config.options().girs_directories().to_owned();
        girs_directories.extend(args.girs_directories().to_owned());

        let package = &config.options().package_file();
        let main_namespace =
            Repository::from_path(find_file_or_fail(package, &args.girs_directories()).unwrap())
                .unwrap();

        let mut namespaces = HashMap::new();
        for include in main_namespace.namespace_includes() {
            if namespaces.contains_key(&include.as_package()) {
                continue;
            }

            let namespace = Repository::from_path(
                find_file_or_fail(&include.as_package_file(), &args.girs_directories()).unwrap(),
            )
            .unwrap();
            namespaces.insert(include.as_package(), namespace);
        }

        Ok(Self {
            repository: main_namespace,
            namespaces,
            args,
            config,
        })
    }

    pub fn generate(&self, mode: cli::Mode) {
        match mode {
            cli::Mode::Doc => todo!(),
            cli::Mode::Normal => todo!(),
            cli::Mode::Sys => {
                codegen::sys::generate(
                    self,
                    std::fs::OpenOptions::new()
                        .create(true)
                        .write(true)
                        .open("./tests/sys/lib.rs")
                        .unwrap(),
                );
            }
            cli::Mode::NotBound => todo!(),
        }
    }
}

fn main() {
    let args = Args::parse();
    //// TODO: normally the fallback value should be from the Gir.toml file
    let mode = args.mode().unwrap();
    let library = Library::new(args).unwrap();
    library.generate(mode);
}
