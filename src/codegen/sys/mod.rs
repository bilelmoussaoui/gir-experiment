use crate::Library;

pub fn generate(library: &Library, dest: impl std::io::Write) {
    println!(
        "Generating ffi for {}",
        library.repository.namespace().name()
    );
    let mut tera = tera::Tera::default();
    tera.add_template_file("src/templates/sys/lib.md", Some("lib.rs"))
        .unwrap();
    tera.add_template_file("src/templates/sys/alias.md", Some("alias"))
        .unwrap();
    tera.add_template_file("src/templates/sys/member.md", Some("member"))
        .unwrap();
    tera.add_template_file("src/templates/sys/record-opaque.md", Some("record-opaque"))
        .unwrap();
    tera.add_template_file("src/templates/sys/record.md", Some("record"))
        .unwrap();

    let mut aliases = vec![];
    for alias in library.repository.namespace().aliases() {
        let mut context = tera::Context::new();
        context.insert("target_type", alias.c_type());
        // TODO: handle the crate name the dest type
        context.insert("dest_type", alias.ty().c_type().unwrap());
        aliases.push(tera.render("alias", &context).unwrap());
    }

    let mut enums = vec![];
    for enumerator in library.repository.namespace().enums() {
        let mut context = tera::Context::new();
        context.insert("target_type", enumerator.c_type());
        context.insert("dest_type", "c_int");
        enums.push(tera.render("alias", &context).unwrap());
        // TODO: handle the crate name the dest type
        for member in enumerator.members() {
            let mut context = tera::Context::new();
            context.insert("name", member.c_identifier());
            context.insert("type", enumerator.c_type());
            context.insert("value", member.value());
            enums.push(tera.render("member", &context).unwrap());
        }
        enums.push("".to_owned());
    }

    let mut constants = vec![];
    for constant in library.repository.namespace().constants() {
        let mut context = tera::Context::new();
        context.insert("name", constant.c_type());
        context.insert("type", ""); // TODO: handle null terminated strings
        context.insert("value", constant.value());
        constants.push(tera.render("member", &context).unwrap());
    }

    let mut flags = vec![];
    for flag in library.repository.namespace().flags() {
        let mut context = tera::Context::new();
        context.insert("target_type", flag.c_type());
        context.insert("dest_type", "c_uint");
        flags.push(tera.render("alias", &context).unwrap());
        // TODO: handle the crate name the dest type
        for member in flag.members() {
            let mut context = tera::Context::new();
            context.insert("name", member.c_identifier());
            context.insert("type", flag.c_type());
            context.insert("value", member.value());
            flags.push(tera.render("member", &context).unwrap());
        }
        flags.push("".to_owned());
    }

    let mut records = vec![];
    for record in library.repository.namespace().records() {
        if record.is_opaque() {
            let mut context = tera::Context::new();
            context.insert("type", record.c_type().unwrap());
            context.insert("is_disguised", &record.is_disguised());
            records.push(tera.render("record-opaque", &context).unwrap());
        } else if record.is_gtype_struct() {
        } else {
            let mut context = tera::Context::new();
            context.insert("type", record.c_type().unwrap());
            let mut fields = vec![];
            for field in record.fields() {
                let mut field_context = tera::Context::new();
                if field.is_type() {
                    let ty = field.as_type();
                    field_context.insert("name", field.name());
                    field_context.insert("type", ty.c_type().unwrap());
                    fields.push(
                        tera.render_str("pub {{name}}: {{type}},", &field_context)
                            .unwrap(),
                    );
                }
            }
            context.insert("fields", &fields);
            records.push(tera.render("record", &context).unwrap());
        }
    }

    let mut context = tera::Context::new();
    context.insert("aliases", &aliases.join("\n"));
    context.insert("enums", &enums.join("\n"));
    context.insert("constants", &constants.join("\n"));
    context.insert("flags", &flags.join("\n"));
    context.insert("records", &records.join("\n"));
    context.insert("link_name", &library.repository.namespace().link_name());
    tera.render_to("lib.rs", &context, dest).unwrap();
}
