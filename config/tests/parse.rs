const CONFIGS: [&str; 11] = [
    "GdkPixbuf-Gir.toml",
    "Gio-Gir.toml",
    "GLib-Gir.toml",
    "GObject-Gir.toml",
    "Graphene-Gir.toml",
    "Gtk4-Gir.toml",
    "GdkPixbuf-sys-Gir.toml",
    "Gio-sys-Gir.toml",
    "GLib-sys-Gir.toml",
    "GObject-sys-Gir.toml",
    "Gst-Gir.toml",
];

#[test]
fn parse_all() {
    for config in CONFIGS {
        gir_config::Config::from_path(&format!("../config/tests/configs/{config}")).unwrap();
    }
}
