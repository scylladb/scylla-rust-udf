use proc_macro2::TokenStream as TokenStream2;
use syn::{AttributeArgs, Error, Lit, Meta, NestedMeta};

// function that returns the value of the "crate" attribute given AttributeArgs
pub(crate) fn get_path_no_internal(atrs: &AttributeArgs) -> Result<TokenStream2, Error> {
    let mut this_path: Option<proc_macro2::TokenStream> = None;
    for attr in atrs.iter() {
        match attr {
            NestedMeta::Lit(lit) => {
                return Err(Error::new_spanned(
                    lit,
                    "unexpected literal attribute for `scylla_udf`",
                ));
            }
            NestedMeta::Meta(meta) => {
                if !meta.path().is_ident("crate") {
                    return Err(Error::new_spanned(
                        meta,
                        "unexpected meta attribute for `scylla_udf`",
                    ));
                }
                match meta {
                    Meta::NameValue(meta_name_value) => {
                        if let Lit::Str(lit_str) = &meta_name_value.lit {
                            let path_val =
                                &lit_str.value().parse::<proc_macro2::TokenStream>().unwrap();
                            if this_path.is_none() {
                                this_path = Some(quote::quote!(#path_val));
                            } else {
                                return Err(syn::Error::new_spanned(
                                    &meta_name_value.lit,
                                    "the `crate` attribute was set multiple times",
                                ));
                            }
                        } else {
                            return Err(syn::Error::new_spanned(
                                &meta_name_value.lit,
                                "the `crate` attribute should be a string literal",
                            ));
                        }
                    }
                    other => {
                        return Err(Error::new_spanned(
                            other,
                            "the `crate` attribute have a single value",
                        ));
                    }
                }
            }
        }
    }
    Ok(this_path.unwrap_or_else(|| quote::quote!(scylla_udf)))
}

pub(crate) fn append_internal(path: &TokenStream2) -> TokenStream2 {
    quote::quote!(#path::_macro_internal)
}

pub(crate) fn get_path(atrs: &AttributeArgs) -> Result<TokenStream2, Error> {
    let path = get_path_no_internal(atrs)?;
    Ok(append_internal(&path))
}
