use std::path::Path;

mod aliases;
mod constants;
mod enums;
use crate::{Library, Result};

fn generate_mod(tera: &tera::Tera, _library: &Library, dest: impl std::io::Write) -> Result<()> {
    tracing::info!("Generating mod.rs");

    let context = tera::Context::new();
    tera.render_to("mod.rs", &context, dest)?;
    Ok(())
}

pub fn generate(library: &Library, dest_dir: impl AsRef<Path>) -> Result<()> {
    let dest_dir = dest_dir.as_ref();
    let namespace = library.repository.namespace();
    tracing::info!(
        "Generating safe wrappers for {}-{}",
        namespace.name(),
        namespace.version()
    );

    let src_dir = dest_dir.join("src").join("auto");
    tracing::debug!("Creating directory: {}", src_dir.display());
    std::fs::create_dir_all(&src_dir)?;

    let mut tera = tera::Tera::default();
    tera.add_template_file("codegen/src/templates/auto/mod.rs", Some("mod.rs"))?;
    tera.add_template_file("codegen/src/templates/auto/enums.rs", Some("enums.rs"))?;
    tera.add_template_file("codegen/src/templates/auto/alias.rs", Some("alias.rs"))?;
    tera.add_template_file(
        "codegen/src/templates/auto/constants.rs",
        Some("constants.rs"),
    )?;

    let dest = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(src_dir.join("mod.rs"))?;
    generate_mod(&tera, library, dest)?;

    let dest = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(src_dir.join("enums.rs"))?;
    enums::generate(&tera, library, dest)?;
    let dest = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(src_dir.join("alias.rs"))?;
    aliases::generate(&tera, library, dest)?;
    let dest = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(src_dir.join("constants.rs"))?;
    constants::generate(&tera, library, dest)?;

    Ok(())
}
