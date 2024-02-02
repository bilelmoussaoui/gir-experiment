use std::{collections::HashMap, path::PathBuf};

use clap::Parser;
use cli::Args;

use crate::parser::repository::Repository;

mod cli;
mod codegen;
mod parser;

pub struct Library {
    repository: Repository,
    namespaces: HashMap<String, Repository>,
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
    pub fn for_package(package: &str, args: Args) -> Result<Self, &'static str> {
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
    // TODO: normally the fallback value should be from the Gir.toml file
    let mode = args.mode().unwrap();
    let library = Library::for_package("Gtk-4.0.gir", args).unwrap();
    library.generate(mode);
}
