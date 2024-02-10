use serde::Serialize;

use crate::{Library, Result};

#[derive(Serialize)]
pub struct Constant {
    name: String,
    c_type: String,
    ty_name: String,
    ty_value: String,
}

pub fn generate(
    tera: &tera::Tera,
    library: &Library,
    dest: impl std::io::Write,
) -> Result<Vec<Constant>> {
    let namespace = library.repository.namespace();
    tracing::info!("Generating constants.rs");

    let mut constants = vec![];
    for constant in namespace.constants() {
        let ty = constant.ty().as_type();
        constants.push(Constant {
            name: constant.name().to_owned(),
            c_type: constant.c_type().to_owned(),
            ty_name: ty.name().unwrap().to_owned(),
            ty_value: ty.c_type().unwrap().to_owned(),
        });
    }

    let mut context = tera::Context::new();
    context.insert("constants", &constants);
    tera.render_to("constants.rs", &context, dest)?;
    Ok(constants)
}
