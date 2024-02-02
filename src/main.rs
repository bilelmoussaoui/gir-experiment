use crate::repository::Repository;

mod alias;
mod array;
mod callback;
mod class;
mod enums;
mod function;
mod function_macro;
mod namespace;
mod property;
mod record;
mod repository;
mod transfer;
mod r#type;
mod union;
mod version;

fn main() {
    let gir_file = "GLib-2.0";
    let content = std::fs::read_to_string(format!("./gir-files/{gir_file}.gir")).unwrap();
    let repo: Repository = quick_xml::de::from_str(&content).unwrap();
    println!("Hello, world! {:#?}", repo);
}
