use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{parse_macro_input, FnArg, ItemFn};

fn get_parameters_and_arguments(
    item: &ItemFn,
    path: &TokenStream2,
) -> Result<(Vec<TokenStream2>, Vec<TokenStream2>), TokenStream2> {
    let inputs = &item.sig.inputs;
    let mut parameters = Vec::with_capacity(inputs.len());
    let mut arguments = Vec::with_capacity(inputs.len());
    for (idx, i) in inputs.iter().enumerate() {
        if let FnArg::Typed(pat) = i {
            let ident = format_ident!("arg_{}", idx);
            let typ = &pat.ty;
            parameters.push(quote! { #ident: <#typ as ::#path::WasmConvertible>::WasmType });
            arguments.push(quote! { <#typ as ::#path::WasmConvertible>::from_wasm(#ident) });
        } else {
            return Err(syn::Error::new(
                i.span(),
                "unexpected untyped self parameter in a scylla_udf function.",
            )
            .to_compile_error());
        }
    }
    Ok((parameters, arguments))
}

fn get_output_type_and_block(
    item: &ItemFn,
    arguments: &[TokenStream2],
    path: &TokenStream2,
) -> Result<(TokenStream2, TokenStream2), TokenStream2> {
    let fun_name = item.sig.ident.clone();
    if let syn::ReturnType::Type(_, typ) = &item.sig.output {
        Ok((
            quote! { -> <#typ as ::#path::WasmConvertible>::WasmType },
            quote! { {
                <#typ as ::#path::WasmConvertible>::to_wasm(&#fun_name(#(#arguments),*))
            } },
        ))
    } else {
        Err(syn::Error::new(
            item.sig.output.span(),
            "scylla_udf function should return a value.",
        )
        .to_compile_error())
    }
}

fn get_exported_fun(
    item: &ItemFn,
    parameters: &[TokenStream2],
    output_type_token: TokenStream2,
    exported_block: TokenStream2,
) -> TokenStream2 {
    let fun_name = &item.sig.ident;
    let exported_fun_ident = format_ident!("{}{}", "_scylla_internal_", fun_name);
    // The exported function doesn't need to be pub, because it will be included in the final
    // binary anyway due to the #[export_name] attribute. No pub helps with the UDT implementation.
    let sig_exported = quote! {
        extern "C" fn #exported_fun_ident(#(#parameters),*) #output_type_token
    };
    let fun_name_string = fun_name.to_string();
    let export_name = quote! {
        #[export_name = #fun_name_string]
    };
    quote! {
        #export_name
        #sig_exported #exported_block
    }
}

/// The macro transforms a function:
/// ```ignore
/// #[scylla_udf::export_udf]
/// fn foo(arg1: u32, arg2: String) -> u32 {
///     arg1 + arg2.len() as u32
/// }
/// ```
/// into something like:
/// ```ignore
/// fn foo(arg1: u32, arg2: String) -> u32 {
///     arg1 + arg2.len() as u32
/// }
/// #[export_name = "foo"]
/// extern "C" fn _scylla_internal_foo(arg1: u32, arg2: WasmPtr) -> u32 {
///     foo(from_wasm(arg1), from_wasm(arg2)).to_wasm()
/// }
/// ```
pub(crate) fn export_udf(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    let atrs = syn::parse_macro_input!(attrs as syn::AttributeArgs);
    let path = crate::path::get_path(&atrs).expect("Couldn't get path to the scylla_udf crate");
    let (parameters, arguments) = match get_parameters_and_arguments(&item, &path) {
        Ok(pa) => pa,
        Err(e) => return e.into(),
    };
    let (output_type_token, exported_block) =
        match get_output_type_and_block(&item, &arguments, &path) {
            Ok(oe) => oe,
            Err(e) => return e.into(),
        };
    let exported_fun = get_exported_fun(&item, &parameters, output_type_token, exported_block);
    quote! {
        #item
        #exported_fun
    }
    .into()
}
