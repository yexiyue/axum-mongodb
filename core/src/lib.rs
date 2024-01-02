use proc_macro::TokenStream;
mod column;
pub(crate) mod column_meta;
use column::collect_meta;
use column_meta::COLUMN_META;
use quote::{quote, ToTokens};
use syn::parse_macro_input;

#[proc_macro_derive(Column)]
pub fn column_derive(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    collect_meta(&st)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let st = parse_macro_input!(input as syn::ItemFn);
    let mut res = proc_macro2::TokenStream::new();
    res.extend(quote!(
        #[axum_mongodb::inject_meta]
        pub struct Servers {}
        #st
    ));
    res.into()
}

#[proc_macro_attribute]
pub fn inject_meta(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let st = parse_macro_input!(input as syn::ItemStruct);
    column::inject_meta(&st)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn inject(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let st = parse_macro_input!(input as syn::ItemFn);
    column::inject(&st)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
