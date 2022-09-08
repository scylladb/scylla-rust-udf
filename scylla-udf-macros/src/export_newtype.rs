use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Fields;

struct NewtypeStruct {
    struct_name: syn::Ident,
    field_type: syn::Type,
    generics: syn::Generics,
}

fn get_newtype_struct(st: &syn::ItemStruct) -> Result<NewtypeStruct, TokenStream2> {
    let struct_name = &st.ident;
    let struct_fields = match &st.fields {
        Fields::Unnamed(named_fields) => named_fields,
        _ => {
            return Err(syn::Error::new_spanned(
                st,
                "#[scylla_udf::export_newtype] error: struct has named fields.",
            )
            .to_compile_error());
        }
    };
    if struct_fields.unnamed.len() > 1 {
        return Err(syn::Error::new_spanned(
            st,
            "#[scylla_udf::export_newtype] error: struct has more than 1 field.",
        )
        .to_compile_error());
    }
    let field_type = match struct_fields.unnamed.first() {
        Some(field) => &field.ty,
        None => {
            return Err(syn::Error::new_spanned(
                st,
                "#[scylla_udf::export_newtype] error: struct has no fields.",
            )
            .to_compile_error());
        }
    };

    Ok(NewtypeStruct {
        struct_name: struct_name.clone(),
        field_type: field_type.clone(),
        generics: st.generics.clone(),
    })
}

fn impl_wasm_convertible(nst: &NewtypeStruct, path: &TokenStream2) -> TokenStream2 {
    let struct_name = &nst.struct_name;
    let struct_type = &nst.field_type;
    let (impl_generics, ty_generics, where_clause) = nst.generics.split_for_impl();
    quote! {
        impl #impl_generics ::#path::WasmConvertible for #struct_name #ty_generics #where_clause {
            type WasmType = <#struct_type as ::#path::WasmConvertible>::WasmType;
            fn from_wasm(arg: Self::WasmType) -> Self {
                #struct_name(<#struct_type as ::#path::WasmConvertible>::from_wasm(arg))
            }
            fn to_wasm(&self) -> Self::WasmType {
                <#struct_type as ::#path::WasmConvertible>::to_wasm(&self.0)
            }
        }
    }
}

fn impl_to_col_type(nst: &NewtypeStruct, path: &TokenStream2) -> TokenStream2 {
    let struct_name = &nst.struct_name;
    let struct_type = &nst.field_type;
    let (impl_generics, ty_generics, where_clause) = nst.generics.split_for_impl();
    quote! {
        impl #impl_generics ::#path::ToColumnType for #struct_name #ty_generics #where_clause {
            fn to_column_type() -> ::#path::ColumnType {
                <#struct_type as ::#path::ToColumnType>::to_column_type()
            }
        }
    }
}

fn impl_value(nst: &NewtypeStruct, path: &TokenStream2) -> TokenStream2 {
    let struct_name = &nst.struct_name;
    let struct_type = &nst.field_type;
    let (impl_generics, ty_generics, where_clause) = nst.generics.split_for_impl();

    quote! {
        impl #impl_generics ::#path::Value for #struct_name #ty_generics #where_clause {
            fn serialize(&self, buf: &mut ::std::vec::Vec<::core::primitive::u8>) -> ::std::result::Result<(), ::#path::ValueTooBig> {
                <#struct_type as ::#path::Value>::serialize(&self.0, buf)
            }
        }
    }
}

fn impl_from_cql_val(nst: &NewtypeStruct, path: &TokenStream2) -> TokenStream2 {
    let struct_name = &nst.struct_name;
    let struct_type = &nst.field_type;
    let (impl_generics, ty_generics, where_clause) = nst.generics.split_for_impl();

    quote! {
        impl #impl_generics ::#path::FromCqlVal<::#path::CqlValue> for #struct_name #ty_generics #where_clause {
            fn from_cql(val: ::#path::CqlValue) -> ::std::result::Result<Self, ::#path::FromCqlValError> {
                <#struct_type as ::#path::FromCqlVal<::#path::CqlValue>>::from_cql(val).map(|v| #struct_name(v))
            }
        }
    }
}

pub(crate) fn export_newtype(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(item as syn::ItemStruct);
    let atrs = syn::parse_macro_input!(attrs as syn::AttributeArgs);
    let path = crate::path::get_path(&atrs).expect("Couldn't get path to the scylla_udf crate");
    let newtype_struct = match get_newtype_struct(&st) {
        Ok(nst) => nst,
        Err(e) => return e.into(),
    };
    let wasm_convertible = impl_wasm_convertible(&newtype_struct, &path);
    let to_col_type = impl_to_col_type(&newtype_struct, &path);
    let value = impl_value(&newtype_struct, &path);
    let from_cql_val = impl_from_cql_val(&newtype_struct, &path);
    quote! {
        #st
        #wasm_convertible
        #to_col_type
        #value
        #from_cql_val
    }
    .into()
}
