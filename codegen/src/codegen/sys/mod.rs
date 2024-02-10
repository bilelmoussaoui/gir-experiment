use std::path::Path;

use crate::{Library, Result};
mod build;
mod lib;

pub fn generate(library: &Library, dest_dir: impl AsRef<Path>) -> Result<()> {
    let dest_dir = dest_dir.as_ref();
    let namespace = library.repository.namespace();
    tracing::info!(
        "Generating ffi for {}-{}",
        namespace.name(),
        namespace.version()
    );

    let src_dir = dest_dir.join("src");
    tracing::debug!("Creating directory: {}", src_dir.display());
    std::fs::create_dir_all(&src_dir)?;

    let mut tera = tera::Tera::default();
    tera.add_template_file("codegen/src/templates/sys/lib.rs", Some("lib.rs"))?;
    tera.add_template_file("codegen/src/templates/sys/build.rs", Some("build.rs"))?;

    let dest = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(src_dir.join("lib.rs"))?;
    lib::generate(&tera, library, dest)?;

    let dest = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(dest_dir.join("build.rs"))?;
    build::generate(&tera, library, dest)?;
    Ok(())
}
