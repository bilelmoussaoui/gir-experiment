use std::{collections::HashMap, path::Path};

use clap::Parser;

use crate::parser::repository::Repository;

mod cli;
mod parser;

pub struct Library {
    repository: Repository,
    namespaces: HashMap<String, Repository>,
}

impl Library {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, ()> {
        let path = path.as_ref();
        // For now assume we have all the gir-files in the same directory
        let gir_files_dir = path.parent().unwrap();
        let main_namespace = Repository::from_path(path)?;
        let mut namespaces = HashMap::new();
        for include in main_namespace.namespace_includes() {
            if namespaces.contains_key(&include.as_package()) {
                continue;
            }

            let gir_file = gir_files_dir.join(include.as_package_file());
            let namespace = Repository::from_path(&gir_file)?;
            namespaces.insert(include.as_package(), namespace);
        }

        Ok(Self {
            repository: main_namespace,
            namespaces,
        })
    }
}

fn main() {
    let args = cli::Args::parse();

    let library = Library::from_path("./gir-files/Gtk-4.0.gir").unwrap();
}
