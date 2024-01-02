use crate::{column_meta::ColumnMeta, COLUMN_META};
use quote::quote;
use syn::{Result, TypePath};

pub fn inject_meta(st: &syn::ItemStruct) -> Result<proc_macro2::TokenStream> {
    let mut fields = proc_macro2::TokenStream::new();
    let mut fields_init = proc_macro2::TokenStream::new();
    let struct_name = &st.ident;
    let vis = &st.vis;
    if !&st.fields.is_empty() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Struct must be empty",
        ));
    }
    for (_, ColumnMeta { name, struct_name }) in unsafe { COLUMN_META.iter() } {
        let field_name = syn::Ident::new(name, proc_macro2::Span::call_site());
        let struct_type: TypePath = syn::parse_str(&struct_name)?;
        fields.extend(quote!(
            pub #field_name:axum_mongodb::Server<#struct_type>,
        ));
        fields_init.extend(quote!(
            #field_name:axum_mongodb::Server::new(db),
        ));
    }

    Ok(quote! {
        #[derive(Clone,Debug)]
        #vis struct #struct_name{
            #fields
        }

        impl axum_mongodb::NewWithDb for #struct_name{
            fn new(db:&mongodb::Database)->Self{
                #struct_name{
                    #fields_init
                }
            }
        }
    })
}
