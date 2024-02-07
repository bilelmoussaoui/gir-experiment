use xmlserde::Unparsed;

use crate::{attribute::Attribute, version::Version, Stability};

pub trait Documentable {
    fn doc(&self) -> Option<&Unparsed>;
    fn doc_deprecated(&self) -> Option<&Unparsed>;
    // TODO: add source-position, doc-stability, doc-version
}

pub trait Attributable {
    fn attributes(&self) -> &[Attribute];

    fn gtk_property_get(&self) -> Option<&str> {
        self.attributes()
            .iter()
            .find(|a| a.name() == "org.gtk.Property.get")
            .map(|a| a.value())
    }

    fn gtk_method_get_property(&self) -> Option<&str> {
        self.attributes()
            .iter()
            .find(|a| a.name() == "org.gtk.Method.get_property")
            .map(|a| a.value())
    }

    fn gtk_property_set(&self) -> Option<&str> {
        self.attributes()
            .iter()
            .find(|a| a.name() == "org.gtk.Property.set")
            .map(|a| a.value())
    }

    fn gtk_method_set_property(&self) -> Option<&str> {
        self.attributes()
            .iter()
            .find(|a| a.name() == "org.gtk.Method.set_property")
            .map(|a| a.value())
    }
}

pub trait Info: Documentable + Attributable {
    fn is_introspectable(&self) -> bool;
    fn is_deprecated(&self) -> bool;
    fn version(&self) -> Option<&Version>;
    fn deprecated_version(&self) -> Option<&Version>;
    fn stability(&self) -> Option<Stability>;
}

pub trait Callable: Info {
    fn name(&self) -> &str;
    fn c_identifier(&self) -> Option<&str>;
    fn shadows(&self) -> Option<&str>;
    fn shadowed_by(&self) -> Option<&str>;
    fn throws(&self) -> bool;
    fn moved_to(&self) -> Option<&str>;
}

macro_rules! impl_documentable {
    ($rust_type:ident) => {
        impl Documentable for $rust_type {
            fn doc(&self) -> Option<&xmlserde::Unparsed> {
                self.doc.as_ref()
            }

            fn doc_deprecated(&self) -> Option<&xmlserde::Unparsed> {
                self.doc_deprecated.as_ref()
            }
        }
    };
}

macro_rules! impl_attributable {
    ($rust_type:ident) => {
        impl Attributable for $rust_type {
            fn attributes(&self) -> &[Attribute] {
                &self.attributes
            }
        }
    };
}

macro_rules! impl_info {
    ($rust_type:ident) => {
        impl Info for $rust_type {
            fn is_introspectable(&self) -> bool {
                self.introspectable.unwrap_or(true)
            }

            fn is_deprecated(&self) -> bool {
                self.deprecated.unwrap_or(false)
            }

            fn version(&self) -> Option<&Version> {
                self.version.as_ref()
            }

            fn deprecated_version(&self) -> Option<&Version> {
                self.deprecated_version.as_ref()
            }

            fn stability(&self) -> Option<Stability> {
                self.stability
            }
        }
    };
}

macro_rules! impl_callable {
    ($rust_type:ident) => {
        impl Callable for $rust_type {
            fn name(&self) -> &str {
                &self.name
            }

            fn c_identifier(&self) -> Option<&str> {
                self.c_identifier.as_deref()
            }

            fn shadows(&self) -> Option<&str> {
                self.shadows.as_deref()
            }

            fn shadowed_by(&self) -> Option<&str> {
                self.shadowed_by.as_deref()
            }

            fn throws(&self) -> bool {
                self.throws.unwrap_or(false)
            }

            fn moved_to(&self) -> Option<&str> {
                self.moved_to.as_deref()
            }
        }
    };
}
