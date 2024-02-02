use std::{collections::HashMap, path::Path};

use crate::parser::repository::Repository;

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
    let gir_files = [
        "Atk-1.0",
        "cairo-1.0",
        "fontconfig-2.0",
        "freetype2-2.0",
        "Gdk-3.0",
        "Gdk-4.0",
        "GdkPixbuf-2.0",
        "GdkPixdata-2.0",
        "GdkWayland-4.0",
        "GdkWin32-4.0",
        "GdkX11-3.0",
        "GdkX11-4.0",
        "Gio-2.0",
        "GL-1.0",
        "GLib-2.0",
        "GModule-2.0",
        "GObject-2.0",
        "Graphene-1.0",
        "Gsk-4.0",
        "Gtk-3.0",
        "Gtk-4.0",
        "HarfBuzz-0.0",
        "libxml2-2.0",
        "Pango-1.0",
        "PangoCairo-1.0",
        "PangoFc-1.0",
        "PangoFT2-1.0",
        "PangoOT-1.0",
        "PangoXft-1.0",
        "Vulkan-1.0",
        "win32-1.0",
        "xfixes-4.0",
        "xft-2.0",
        "xlib-2.0",
        "xrandr-1.3",
    ];

    //for gir_file in gir_files {
    //    //    let gir_file = "Gtk-3.0";
    //    let _library = Library::from_path(format!("./gir-files/{gir_file}.gir")).unwrap();
    //    //println!("Hello, world! {:#?}", repo);
    //}

    let library = Library::from_path("./gir-files/Gtk-4.0.gir").unwrap();
}
