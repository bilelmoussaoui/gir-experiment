use crate::Library;

pub fn generate(library: &Library) {
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

    let mut context = tera::Context::new();
    context.insert("aliases", &aliases.join("\n"));
    context.insert("enums", &enums.join("\n"));
    context.insert("constants", &constants.join("\n"));
    context.insert("flags", &flags.join("\n"));
    println!("{}", tera.render("lib.rs", &context).unwrap());
}
