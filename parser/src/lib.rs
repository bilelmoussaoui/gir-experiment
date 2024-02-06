use xmlserde::xml_serde_enum;

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("I/O operation failed")]
    IO(#[from] std::io::Error),
    #[error("Failed to parse xml file: {0}")]
    Xml(String),
}

xml_serde_enum! {
    #[derive(Debug, Copy, Clone)]
    Stability {
        Stable => "Stable",
        Unstable => "Unstable",
        Private => "Private",
    }
}

xml_serde_enum! {
    #[derive(Debug, Copy, Clone)]
    TransferOwnership {
        None => "none",
        Container => "container",
        Full => "full",
    }
}

impl TransferOwnership {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

xml_serde_enum! {
    #[derive(Debug, Copy, Clone)]
    FunctionScope {
        Call => "call",
        Notified => "notified",
        Async => "async",
        Forever => "forever",
    }
}

xml_serde_enum! {
    #[derive(Debug, Copy, Clone)]
    SignalEmission {
        First => "first",
        Last => "last",
        Cleanup => "cleanup",
    }
}

#[macro_use]
mod traits;
pub mod prelude {
    pub use xmlserde::XmlValue;

    pub use super::traits::*;
}

pub mod alias;
pub mod array;
pub mod attribute;
pub mod bitfield;
pub mod boxed;
pub mod callback;
pub mod class;
pub mod constant;
pub mod enums;
pub mod field;
pub mod function;
pub mod function_macro;
pub mod interface;
pub mod member;
pub mod method;
pub mod namespace;
pub mod parameter;
pub mod property;
pub mod record;
pub mod repository;
pub mod return_value;
pub mod signal;
pub mod r#type;
pub mod union;
pub mod version;
pub mod virtual_method;

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use super::prelude::*;

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

    use super::{repository::Repository, version::Version};

    fn parse_gir(gir_file: &str) -> Repository {
        let path = PathBuf::from("../gir-files").join(&format!("{}.gir", gir_file));
        Repository::from_path(path).unwrap()
    }

    #[test]
    fn pango_xft_gir() {
        let repo = parse_gir(GIR_FILES[28]);
        assert_eq!(repo.version(), Version::from_str("1.2").ok().as_ref());
        assert_eq!(repo.namespace_includes().len(), 6);
        assert_eq!(repo.namespace_includes()[0].as_package(), "GObject-2.0");
        assert_eq!(repo.packages()[0].name(), "pangoxft");
        assert_eq!(repo.header_includes()[0].name(), "pango/pangoxft.h");

        let namespace = repo.namespace();
        assert_eq!(namespace.version(), "1.0");
        assert_eq!(namespace.name(), "PangoXft");
        assert_eq!(namespace.shared_library(), Some("libpangoxft-1.0.so.0"));
        assert_eq!(namespace.c_identifier_prefixes(), "PangoXft");
        assert_eq!(namespace.c_symbol_prefixes(), "pango_xft");

        let macros = namespace.macros();
        assert_eq!(macros[0].name(), "FONT");
        assert_eq!(macros[0].c_identifier(), Some("PANGO_XFT_FONT"));
        assert!(!macros[0].is_introspectable());
        assert!(!macros[0].parameters().is_empty());

        let classes = namespace.classes();
        assert_eq!(classes[0].name(), "Font");
        assert_eq!(classes[0].symbol_prefix(), Some("font"));
        assert_eq!(classes[0].c_type(), Some("PangoXftFont"));
        assert_eq!(classes[0].parent(), Some("PangoFc.Font"));
        assert_eq!(classes[0].g_type_name(), "PangoXftFont");
        assert_eq!(classes[0].g_get_type(), "pango_xft_font_get_type");

        assert_eq!(classes[2].name(), "Renderer");
        assert_eq!(classes[2].symbol_prefix(), Some("renderer"));
        assert_eq!(classes[2].c_type(), Some("PangoXftRenderer"));
        assert_eq!(classes[2].parent(), Some("Pango.Renderer"));
        assert_eq!(classes[2].g_type_name(), "PangoXftRenderer");
        assert_eq!(classes[2].g_get_type(), "pango_xft_renderer_get_type");
        assert_eq!(classes[2].g_type_struct(), Some("RendererClass"));
        assert_eq!(classes[2].version(), Version::from_str("1.8").ok().as_ref());

        let virtual_methods = classes[2].virtual_methods();
        assert_eq!(virtual_methods[0].name(), "composite_glyphs");
        let return_value = virtual_methods[0].return_value();
        assert!(return_value.transfer_ownership().unwrap().is_none());
        assert_eq!(return_value.ty().name(), Some("none"));
        assert_eq!(return_value.ty().c_type(), Some("void"));

        let properties = classes[2].properties();
        assert_eq!(properties[0].name(), "display");
        assert!(!properties[0].is_readable());
        assert!(properties[0].is_writable());
        assert!(properties[0].is_construct_only());
        assert!(properties[0].transfer_ownership().is_none());
        assert_eq!(properties[0].ty().name(), Some("gpointer"));
        assert_eq!(properties[0].ty().c_type(), Some("gpointer"));

        assert_eq!(properties[1].name(), "screen");
        assert!(!properties[1].is_readable());
        assert!(properties[1].is_writable());
        assert!(properties[1].is_construct_only());
        assert!(properties[1].transfer_ownership().is_none());
        assert_eq!(properties[1].ty().name(), Some("gint"));
        assert_eq!(properties[1].ty().c_type(), Some("gint"));

        let fields = classes[2].fields();
        assert_eq!(fields[0].name(), "parent_instance");
        assert!(!fields[0].is_readable());
        assert!(fields[0].is_private());
        assert_eq!(fields[0].ty().as_type().name(), Some("Pango.Renderer"));
        assert_eq!(fields[0].ty().as_type().c_type(), Some("PangoRenderer"));

        let constructors = classes[2].constructors();
        assert_eq!(constructors[0].name(), "new");
        assert_eq!(
            constructors[0].c_identifier(),
            Some("pango_xft_renderer_new")
        );
        assert_eq!(
            constructors[0].version(),
            Version::from_str("1.8").ok().as_ref()
        );
    }

    #[test]
    fn xft_gir() {
        let repo = parse_gir(GIR_FILES[32]);
        assert_eq!(repo.version(), Version::from_str("1.2").ok().as_ref());
        assert_eq!(repo.namespace_includes()[0].as_package(), "xlib-2.0");

        let namespace = repo.namespace();
        assert_eq!(namespace.version(), "2.0");
        assert_eq!(namespace.name(), "xft");
        assert_eq!(namespace.c_identifier_prefixes(), "Xft");
        assert_eq!(namespace.c_symbol_prefixes(), "Xft");
    }

    #[test]
    fn xlib_gir() {
        let repo = parse_gir(GIR_FILES[33]);
        assert_eq!(repo.version(), Version::from_str("1.2").ok().as_ref());
        let namespace = repo.namespace();
        assert_eq!(namespace.version(), "2.0");
        assert_eq!(namespace.name(), "xlib");
        assert_eq!(namespace.c_identifier_prefixes(), "");
        assert_eq!(namespace.c_symbol_prefixes(), "X");
        let aliases = namespace.aliases();
        assert_eq!(aliases[0].name(), "Atom");
        assert_eq!(aliases[0].c_type(), "Atom");
        assert_eq!(aliases[0].ty().unwrap().as_type().name(), Some("gulong"));
        assert_eq!(aliases[0].ty().unwrap().as_type().c_type(), Some("gulong"));

        let records = namespace.records();
        assert_eq!(records[0].name(), Some("Display"));
        assert_eq!(records[0].c_type(), Some("Display"));

        let unions = namespace.unions();
        assert_eq!(unions[0].name(), Some("XEvent"));
        assert_eq!(unions[0].c_type(), Some("XEvent"));

        let functions = namespace.functions();
        assert_eq!(functions[0].name(), "open_display");
        assert_eq!(functions[0].c_identifier(), Some("XOpenDisplay"));
        assert!(functions[0].parameters().is_empty());

        let return_value = functions[0].return_value();
        assert!(return_value.transfer_ownership().unwrap().is_none());
        assert_eq!(return_value.ty().name(), Some("none"));
        assert_eq!(return_value.ty().c_type(), Some("void"));
    }

    #[test]
    fn xrandr_gir() {
        let repo = parse_gir(GIR_FILES[34]);
        assert_eq!(repo.version(), Version::from_str("1.2").ok().as_ref());
        let namespace = repo.namespace();
        assert_eq!(namespace.version(), "1.3");
        assert_eq!(namespace.name(), "xrandr");
        assert_eq!(namespace.c_identifier_prefixes(), "XRR");
        assert_eq!(namespace.c_symbol_prefixes(), "XRR");
        let records = namespace.records();
        assert_eq!(records[0].name(), Some("ScreenSize"));
        assert_eq!(records[0].c_type(), Some("XRRScreenSize"));
    }

    #[test]
    fn parse_all_gir_files() {
        for gir_file in GIR_FILES {
            println!("{}", gir_file);
            let repository = parse_gir(gir_file);
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
