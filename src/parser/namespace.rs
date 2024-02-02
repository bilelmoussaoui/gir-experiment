use xmlserde_derives::XmlDeserialize;

use super::{
    alias::Alias, bitfield::BitField, class::Class, constant::Constant, enums::Enumeration,
    function::Function, function_macro::FunctionMacro, interface::Interface, record::Record,
    union::Union,
};

#[derive(Debug, XmlDeserialize)]
#[xmlserde(root = b"namespace")]
pub struct Namespace {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"version", ty = "attr")]
    version: String,
    #[xmlserde(name = b"c:identifier-prefixes", ty = "attr")]
    c_identifier_prefix: String,
    #[xmlserde(name = b"c:symbol-prefixes", ty = "attr")]
    c_symbol_prefix: String,
    #[xmlserde(name = b"shared-library", ty = "attr")]
    shared_library: Option<String>,
    #[xmlserde(name = b"alias", ty = "child")]
    aliases: Vec<Alias>,
    #[xmlserde(name = b"constant", ty = "child")]
    constants: Vec<Constant>,
    #[xmlserde(name = b"union", ty = "child")]
    unions: Vec<Union>,
    #[xmlserde(name = b"function", ty = "child")]
    functions_global: Vec<Function>,
    #[xmlserde(name = b"function-macro", ty = "child")]
    functions_macro: Vec<FunctionMacro>,
    #[xmlserde(name = b"record", ty = "child")]
    records: Vec<Record>,
    #[xmlserde(name = b"enumeration", ty = "child")]
    enums: Vec<Enumeration>,
    #[xmlserde(name = b"bitfield", ty = "child")]
    flags: Vec<BitField>,
    #[xmlserde(name = b"class", ty = "child")]
    classes: Vec<Class>,
    #[xmlserde(name = b"interface", ty = "child")]
    interfaces: Vec<Interface>,
}

impl Namespace {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn aliases(&self) -> &[Alias] {
        &self.aliases
    }

    pub fn enums(&self) -> &[Enumeration] {
        &self.enums
    }

    pub fn constants(&self) -> &[Constant] {
        &self.constants
    }

    pub fn flags(&self) -> &[BitField] {
        &self.flags
    }
}
