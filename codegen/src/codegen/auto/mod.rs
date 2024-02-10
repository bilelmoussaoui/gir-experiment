use std::path::Path;

mod aliases;
mod constants;
mod enums;
use crate::{Library, Result};

fn generate_mod(
    tera: &tera::Tera,
    context: &tera::Context,
    _library: &Library,
    dest: impl std::io::Write,
) -> Result<()> {
    tracing::info!("Generating mod.rs");

    tera.render_to("mod.rs", context, dest)?;
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
        .open(src_dir.join("enums.rs"))?;
    let enums_output = enums::generate(&tera, library, dest)?;
    let dest = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(src_dir.join("alias.rs"))?;
    let aliases_output = aliases::generate(&tera, library, dest)?;
    let dest = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(src_dir.join("constants.rs"))?;
    let constants_output = constants::generate(&tera, library, dest)?;

    let mut context = tera::Context::new();
    context.insert("enums", &enums_output);
    context.insert("flags", &false);
    context.insert("aliases", &aliases_output);
    context.insert("global_functions", &false);
    context.insert("constants", &constants_output);
    context.insert("traits", &false);
    context.insert("builders", &false);

    let dest = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(src_dir.join("mod.rs"))?;
    generate_mod(&tera, &context, library, dest)?;

    Ok(())
}
