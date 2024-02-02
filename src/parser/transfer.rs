use xmlserde::xml_serde_enum;

xml_serde_enum! {
    #[derive(Debug, Clone)]
    TransferOwnership{
        None => "none",
        Container => "container",
        Full => "full",
    }
}
