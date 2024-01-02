use crate::column_meta::{ColumnMeta, COLUMN_META};
use std::borrow::BorrowMut;
use syn::Result;
mod inject_meta;
mod inject;
pub use inject_meta::inject_meta;
pub use inject::inject;

pub fn collect_meta(st: &syn::DeriveInput) -> Result<proc_macro2::TokenStream> {
    let res = proc_macro2::TokenStream::new();
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(_fields_named),
        ..
    }) = &st.data
    {
        let mut name = st.ident.to_string().to_lowercase();
        name.push('s');
        unsafe {
            match COLUMN_META.get(&name) {
                Some(_) => {
                    return Err(syn::Error::new_spanned(
                        &st.ident,
                        format!("Column {} already exists", &st.ident.to_string()),
                    ))
                }
                None => {
                    COLUMN_META
                        .borrow_mut()
                        .insert(name.clone(), ColumnMeta::new(name, st.ident.to_string()));
                }
            }
        }
    } else {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Only named struct is supported",
        ));
    }
    Ok(res)
}
