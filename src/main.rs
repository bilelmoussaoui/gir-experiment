use std::path::Path;

use crate::parser::repository::Repository;

mod parser;

pub struct Library {
    repository: Repository,
}

impl Library {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, ()> {
        println!("Parsing {}", path.as_ref().display());
        let content = std::fs::read_to_string(path).unwrap();
        let repository = xmlserde::xml_deserialize_from_str(&content).unwrap();
        //println!("{:#?}", repository);
        Ok(Self { repository })
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

    for gir_file in gir_files {
        //    let gir_file = "Gtk-3.0";
        let library = Library::from_path(format!("./gir-files/{gir_file}.gir")).unwrap();
        //println!("Hello, world! {:#?}", repo);
    }

    //let library = Library::from_path("./gir-files/Gtk-4.0.gir").unwrap();
}
