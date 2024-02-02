pub mod alias;
pub mod array;
pub mod bitfield;
pub mod callback;
pub mod class;
pub mod constant;
pub mod enums;
pub mod function;
pub mod function_macro;
pub mod interface;
pub mod namespace;
pub mod property;
pub mod record;
pub mod repository;
pub mod signal;
pub mod transfer;
pub mod r#type;
pub mod union;
pub mod version;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::parser::repository::Repository;

    #[test]
    fn parse_all_gir_files() {
        const GIR_FILES: [&str; 35] = [
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

        for gir_file in GIR_FILES {
            let path = PathBuf::from("./gir-files").join(&format!("{}.gir", gir_file));
            let repository = Repository::from_path(path).unwrap();
            assert_eq!(
                gir_file,
                format!(
                    "{}-{}",
                    repository.namespace().name(),
                    repository.namespace().version()
                )
            );
        }
    }
}
