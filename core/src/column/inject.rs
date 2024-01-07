use quote::quote;
use syn::{FnArg, Result, TypePath};

// 用于修改函数签名，方便用户使用
pub fn inject(st: &syn::ItemFn) -> Result<proc_macro2::TokenStream> {
    let mut res = proc_macro2::TokenStream::new();
    let mut st = st.clone();
    change_args(&mut st);
    res.extend(quote! {
        #st
    });
    Ok(res)
}

fn change_args(st: &mut syn::ItemFn) {
    for item in st.sig.inputs.iter_mut() {
        if let FnArg::Typed(pat_type) = item {
            if let syn::Type::Path(TypePath { path, .. }) = pat_type.ty.as_ref() {
                if path.is_ident("DBServers") {
                    pat_type.ty = syn::parse_str("MongoDbServer<crate::Servers>").unwrap();
                }
            }
        }
    }
}
