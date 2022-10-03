use syn::{Data, DeriveInput, Fields, FieldsNamed};
use syn::{Lit, Meta};

/// Parses the tokens_input to a DeriveInput and returns the struct name from which it derives and
/// the named fields
pub(crate) fn parse_named_fields<'a>(
    input: &'a DeriveInput,
    current_derive: &str,
) -> &'a FieldsNamed {
    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(named_fields) => named_fields,
            _ => panic!(
                "derive({}) works only for structs with named fields. Tuples don't need derive.",
                current_derive
            ),
        },
        _ => panic!("derive({}) works only on structs!", current_derive),
    }
}

pub(crate) fn get_path(input: &DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut this_path: Option<proc_macro2::TokenStream> = None;
    for attr in input.attrs.iter() {
        match attr.parse_meta() {
            Ok(Meta::NameValue(meta_name_value)) => {
                if !meta_name_value.path.is_ident("scylla_crate") {
                    continue;
                }
                if let Lit::Str(lit_str) = &meta_name_value.lit {
                    let path_val = &lit_str.value().parse::<proc_macro2::TokenStream>().unwrap();
                    if this_path.is_none() {
                        this_path = Some(quote::quote!(#path_val));
                    } else {
                        return Err(syn::Error::new_spanned(
                            &meta_name_value.lit,
                            "the `scylla_crate` attribute was set multiple times",
                        ));
                    }
                } else {
                    return Err(syn::Error::new_spanned(
                        &meta_name_value.lit,
                        "the `scylla_crate` attribute should be a string literal",
                    ));
                }
            }
            Ok(other) => {
                if !other.path().is_ident("scylla_crate") {
                    continue;
                }
                return Err(syn::Error::new_spanned(
                    other,
                    "the `scylla_crate` attribute have a single value",
                ));
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
    match this_path {
        Some(path) => Ok(path),
        None => Ok(quote::quote!(scylla)),
    }
}
