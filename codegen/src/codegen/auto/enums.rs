use serde::Serialize;

use crate::{codegen::nameutils::enum_member_name, Library, Result};

#[derive(Serialize)]
pub struct Member {
    name: String,
    c_identifier: String,
}

#[derive(Serialize)]
pub struct Enum {
    name: String,
    c_type: String,
    g_get_type: Option<String>,
    members: Vec<Member>,
}

pub fn generate(
    tera: &tera::Tera,
    library: &Library,
    dest: impl std::io::Write,
) -> Result<Vec<Enum>> {
    let namespace = library.repository.namespace();
    tracing::info!("Generating enums.rs");

    let mut enums = vec![];
    for enumerator in namespace.enums() {
        let mut members = vec![];
        for member in enumerator.members() {
            members.push(Member {
                name: enum_member_name(member.name()),
                c_identifier: member.c_identifier().to_owned(),
            });
        }
        enums.push(Enum {
            name: enumerator.name().to_owned(),
            c_type: enumerator.c_type().to_owned(),
            g_get_type: enumerator.g_get_type().map(ToOwned::to_owned),
            members,
        });
    }

    let mut context = tera::Context::new();
    context.insert("enums", &enums);
    tera.render_to("enums.rs", &context, dest)?;
    Ok(enums)
}
