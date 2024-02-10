use serde::Serialize;

use crate::{Library, Result};

#[derive(Serialize)]
pub struct Alias {
    name: String,
    c_type: String,
}

pub fn generate(tera: &tera::Tera, library: &Library, dest: impl std::io::Write) -> Result<()> {
    let namespace = library.repository.namespace();
    tracing::info!("Generating alias.rs");

    let mut aliases = vec![];
    for alias in namespace.aliases() {
        // TODO: handle the dest_type
        aliases.push(Alias {
            name: alias.name().to_owned(),
            c_type: alias.c_type().to_owned(),
        });
    }

    let mut context = tera::Context::new();
    context.insert("aliases", &aliases);
    tera.render_to("alias.rs", &context, dest)?;
    Ok(())
}
