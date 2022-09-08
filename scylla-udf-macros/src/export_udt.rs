use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::Fields;

pub fn impl_wasm_convertible(st: &syn::ItemStruct, path: &TokenStream2) -> TokenStream2 {
    let struct_name = &st.ident;
    let (impl_generics, ty_generics, where_clause) = st.generics.split_for_impl();
    quote! {
        impl #impl_generics ::#path::WasmConvertible for #struct_name #ty_generics #where_clause {
            type WasmType = ::#path::WasmPtr;
            fn from_wasm(arg: Self::WasmType) -> Self {
                <Self as ::#path::FromWasmPtr>::from_wasmptr(arg)
            }
            fn to_wasm(&self) -> Self::WasmType {
                <Self as ::#path::ToWasmPtr>::to_wasmptr(self)
            }
        }
    }
}

pub fn impl_to_col_type(st: &syn::ItemStruct, path: &TokenStream2) -> TokenStream2 {
    let struct_name = &st.ident;
    let struct_name_string = struct_name.to_string();
    let struct_fields = match &st.fields {
        Fields::Named(named_fields) => named_fields,
        _ => {
            return syn::Error::new_spanned(
                st,
                "#[scylla_udf::export_udt] works only for structs with named fields.",
            )
            .to_compile_error()
        }
    };
    let (impl_generics, ty_generics, where_clause) = st.generics.split_for_impl();
    let fields_column_types = struct_fields.named.iter().map(|field| {
        // we matched with Fields::Named above, so we can unwrap
        let field_name = field.ident.as_ref().unwrap().to_string();
        let field_type = &field.ty;
        quote_spanned! {field.span() =>
            (#field_name.to_string(), <#field_type as ::#path::ToColumnType>::to_column_type()),
        }
    });
    quote! {
        impl #impl_generics ::#path::ToColumnType for #struct_name #ty_generics #where_clause {
            fn to_column_type() -> ::#path::ColumnType {
                use ::std::string::ToString;
                ::#path::ColumnType::UserDefinedType {
                    type_name: #struct_name_string.to_string(),
                    keyspace: "".to_string(),
                    field_types: ::std::vec![#(#fields_column_types)*],
                }
            }
        }
    }
}

pub(crate) fn export_udt(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(item as syn::ItemStruct);
    let atrs = syn::parse_macro_input!(attrs as syn::AttributeArgs);
    let path_no_internal = crate::path::get_path_no_internal(&atrs)
        .expect("Couldn't get path to the scylla_udf crate");
    let path_string = path_no_internal.to_string();
    let path = crate::path::append_internal(&path_no_internal);
    let wasm_convertible = impl_wasm_convertible(&st, &path);
    let to_col_type = impl_to_col_type(&st, &path);
    quote! {
        #[derive(::#path::FromUserType)]
        #[derive(::#path::IntoUserType)]
        #[scylla_crate = #path_string]
        #st
        #wasm_convertible
        #to_col_type
    }
    .into()
}
