use xmlserde::Unparsed;
use xmlserde_derives::XmlDeserialize;

use crate::{
    attribute::Attribute,
    callback::Callback,
    constant::Constant,
    field::Field,
    function::{Function, FunctionInline},
    method::{Method, MethodInline},
    prelude::*,
    property::Property,
    record::Record,
    signal::Signal,
    union::Union,
    version::Version,
    virtual_method::VirtualMethod,
    Stability,
};

#[derive(Debug, XmlDeserialize)]
pub struct Implements {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
}

impl Implements {
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, XmlDeserialize)]
pub struct Class {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"c:symbol-prefix", ty = "attr")]
    symbol_prefix: Option<String>,
    #[xmlserde(name = b"c:type", ty = "attr")]
    c_type: Option<String>,
    #[xmlserde(name = b"parent", ty = "attr")]
    parent: Option<String>,
    // Common attributes
    #[xmlserde(name = b"introspectable", ty = "attr")]
    introspectable: Option<bool>,
    #[xmlserde(name = b"deprecated", ty = "attr")]
    deprecated: Option<bool>,
    #[xmlserde(name = b"version", ty = "attr")]
    version: Option<Version>,
    #[xmlserde(name = b"deprecated-version", ty = "attr")]
    deprecated_version: Option<Version>,
    #[xmlserde(name = b"stability", ty = "attr")]
    stability: Option<Stability>,
    // Documentation
    #[xmlserde(name = b"doc", ty = "child")]
    doc: Option<Unparsed>,
    #[xmlserde(name = b"doc-deprecated", ty = "child")]
    doc_deprecated: Option<Unparsed>,
    // Attributes: 0 or more
    #[xmlserde(name = b"attribute", ty = "child")]
    attributes: Vec<Attribute>,

    #[xmlserde(name = b"glib:type-name", ty = "attr")]
    g_type_name: String,
    #[xmlserde(name = b"glib:get-type", ty = "attr")]
    g_get_type: String,
    #[xmlserde(name = b"glib:type-struct", ty = "attr")]
    g_type_struct: Option<String>,
    #[xmlserde(name = b"glib:fundamental", ty = "attr")]
    g_fundamental: Option<bool>,
    #[xmlserde(name = b"final", ty = "attr")]
    r#final: Option<bool>,
    #[xmlserde(name = b"abstract", ty = "attr")]
    r#abstract: Option<bool>,
    #[xmlserde(name = b"glib:ref-func", ty = "attr")]
    g_ref_func: Option<String>,
    #[xmlserde(name = b"glib:unref-func", ty = "attr")]
    g_unref_func: Option<String>,
    #[xmlserde(name = b"glib:set-value-func", ty = "attr")]
    g_set_value_func: Option<String>,
    #[xmlserde(name = b"glib:get-value-func", ty = "attr")]
    g_get_value_func: Option<String>,
    #[xmlserde(name = b"implements", ty = "child")]
    implements: Vec<Implements>,
    #[xmlserde(name = b"constructor", ty = "child")]
    constructors: Vec<Function>,
    #[xmlserde(name = b"function", ty = "child")]
    functions: Vec<Function>,
    #[xmlserde(name = b"function-inline", ty = "child")]
    inline_functions: Vec<FunctionInline>,

    #[xmlserde(name = b"method", ty = "child")]
    methods: Vec<Method>,
    #[xmlserde(name = b"inline-methods", ty = "child")]
    inline_methods: Vec<MethodInline>,

    #[xmlserde(name = b"property", ty = "child")]
    properties: Vec<Property>,
    #[xmlserde(name = b"signal", ty = "child")]
    signals: Vec<Signal>,
    #[xmlserde(name = b"virtual-method", ty = "child")]
    virtual_methods: Vec<VirtualMethod>,
    #[xmlserde(name = b"field", ty = "child")]
    fields: Vec<Field>,
    #[xmlserde(name = b"record", ty = "child")]
    records: Vec<Record>,
    #[xmlserde(name = b"constant", ty = "child")]
    constants: Vec<Constant>,
    #[xmlserde(name = b"union", ty = "child")]
    unions: Vec<Union>,
    #[xmlserde(name = b"callback", ty = "child")]
    callbacks: Vec<Callback>,
}

impl Class {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_fundamental(&self) -> bool {
        self.g_fundamental.unwrap_or(false)
    }

    pub fn is_final(&self) -> bool {
        self.r#final.unwrap_or(false)
    }

    pub fn is_abstract(&self) -> bool {
        self.r#abstract.unwrap_or(false)
    }

    pub fn symbol_prefix(&self) -> Option<&str> {
        self.symbol_prefix.as_deref()
    }

    pub fn c_type(&self) -> Option<&str> {
        self.c_type.as_deref()
    }

    pub fn parent(&self) -> Option<&str> {
        self.parent.as_deref()
    }

    pub fn g_type_name(&self) -> &str {
        &self.g_type_name
    }

    pub fn g_get_type(&self) -> &str {
        &self.g_get_type
    }

    pub fn g_type_struct(&self) -> Option<&str> {
        self.g_type_struct.as_deref()
    }

    pub fn g_ref_func(&self) -> Option<&str> {
        self.g_ref_func.as_deref()
    }

    pub fn g_unref_func(&self) -> Option<&str> {
        self.g_unref_func.as_deref()
    }

    pub fn g_set_value_func(&self) -> Option<&str> {
        self.g_set_value_func.as_deref()
    }

    pub fn g_get_value_func(&self) -> Option<&str> {
        self.g_get_value_func.as_deref()
    }

    pub fn implements(&self) -> &[Implements] {
        &self.implements
    }

    pub fn constructors(&self) -> &[Function] {
        &self.constructors
    }

    pub fn methods(&self) -> &[Method] {
        &self.methods
    }

    pub fn inlined_methods(&self) -> &[MethodInline] {
        &self.inline_methods
    }

    pub fn functions(&self) -> &[Function] {
        &self.functions
    }

    pub fn inlined_functions(&self) -> &[FunctionInline] {
        &self.inline_functions
    }

    pub fn virtual_methods(&self) -> &[VirtualMethod] {
        &self.virtual_methods
    }

    pub fn fields(&self) -> &[Field] {
        &self.fields
    }

    pub fn properties(&self) -> &[Property] {
        &self.properties
    }

    pub fn signals(&self) -> &[Signal] {
        &self.signals
    }

    pub fn unions(&self) -> &[Union] {
        &self.unions
    }

    pub fn constants(&self) -> &[Constant] {
        &self.constants
    }

    pub fn records(&self) -> &[Record] {
        &self.records
    }

    pub fn callbacks(&self) -> &[Callback] {
        &self.callbacks
    }
}

impl_documentable!(Class);
impl_attributable!(Class);
impl_info!(Class);
