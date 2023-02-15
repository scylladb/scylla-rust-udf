use proc_macro::TokenStream;

mod export_newtype;
mod export_udf;
mod export_udt;

#[proc_macro_attribute]
pub fn export_udt(attrs: TokenStream, item: TokenStream) -> TokenStream {
    export_udt::export_udt(attrs, item)
}

#[proc_macro_attribute]
pub fn export_udf(attrs: TokenStream, item: TokenStream) -> TokenStream {
    export_udf::export_udf(attrs, item)
}

#[proc_macro_attribute]
pub fn export_newtype(attrs: TokenStream, item: TokenStream) -> TokenStream {
    export_newtype::export_newtype(attrs, item)
}

pub(crate) mod path;
