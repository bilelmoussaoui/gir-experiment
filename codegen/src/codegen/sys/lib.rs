use serde::Serialize;

use crate::{Library, Result};

#[derive(Serialize)]
pub struct Alias {
    target_type: String,
    dest_type: String,
}

#[derive(Serialize)]
pub struct Member {
    name: String,
    value: String,
}

#[derive(Serialize)]
pub struct Enum {
    target_type: String,
    dest_type: String,
    members: Vec<Member>,
}

#[derive(Serialize)]
pub struct Constant {
    name: String,
    r#type: String,
    value: String,
}

#[derive(Serialize)]
struct Record {
    r#type: String,
    is_disguised: bool,
    is_opaque: bool,
}

pub fn generate(tera: &tera::Tera, library: &Library, dest: impl std::io::Write) -> Result<()> {
    let namespace = library.repository.namespace();
    tracing::info!("Generating lib.rs");

    let mut aliases = vec![];
    for alias in namespace.aliases() {
        aliases.push(Alias {
            target_type: alias.c_type().to_owned(),
            dest_type: alias.ty().unwrap().as_type().c_type().unwrap().to_owned(),
        });
    }

    let mut enums = vec![];
    for enumerator in namespace.enums() {
        let mut members = vec![];
        for member in enumerator.members() {
            members.push(Member {
                name: member.c_identifier().to_owned(),
                value: member.value().to_owned(),
            });
        }
        enums.push(Enum {
            target_type: enumerator.c_type().to_owned(),
            // TODO: handle the crate name the dest type
            dest_type: "c_int".to_owned(),
            members,
        });
    }

    let mut constants = vec![];
    for constant in namespace.constants() {
        constants.push(Constant {
            name: constant.c_type().to_owned(),
            r#type: "".to_owned(), // TODO: handle null terminated strings
            value: constant.value().to_owned(),
        });
    }

    let mut flags = vec![];
    for flag in namespace.flags() {
        let mut members = vec![];
        for member in flag.members() {
            members.push(Member {
                name: member.c_identifier().to_owned(),
                value: member.value().to_owned(),
            });
        }
        flags.push(Enum {
            target_type: flag.c_type().to_owned(),
            // TODO: handle the crate name the dest type
            dest_type: "c_uint".to_owned(),
            members,
        });
    }

    let mut records = vec![];
    for record in namespace.records() {
        records.push(Record {
            r#type: record.c_type().unwrap().to_owned(),
            is_disguised: record.is_disguised(),
            is_opaque: record.is_opaque(),
        });
    }

    let mut context = tera::Context::new();
    context.insert("aliases", &aliases);
    context.insert("enums", &enums);
    context.insert("constants", &constants);
    context.insert("flags", &flags);
    context.insert("records", &records);
    context.insert("link_name", &namespace.link_name());
    tera.render_to("lib.rs", &context, dest)?;
    Ok(())
}
