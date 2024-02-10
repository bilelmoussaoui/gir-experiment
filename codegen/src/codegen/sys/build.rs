use crate::{Library, Result};

pub fn generate(tera: &tera::Tera, _library: &Library, dest: impl std::io::Write) -> Result<()> {
    tracing::info!("Generating build.rs");

    let context = tera::Context::new();
    tera.render_to("build.rs", &context, dest)?;
    Ok(())
}
