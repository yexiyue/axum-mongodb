use crate::{
    column_meta::{ColumnMeta, COLUMN_META},
    indexes::Indexes,
};
use quote::quote;
use std::borrow::BorrowMut;
use syn::Result;
mod inject;
mod inject_meta;
pub use inject::inject;
pub use inject_meta::inject_meta;

// 收集元信息，并生成代码，实现Server<T>
pub fn collect_meta(st: &syn::DeriveInput, drop: bool) -> Result<proc_macro2::TokenStream> {
    let mut res = proc_macro2::TokenStream::new();
    let mut field_index = Vec::new();
    
    let mut name = st.ident.to_string().to_lowercase();
    name.push('s');
    if let syn::Data::Struct(syn::DataStruct { fields, .. }) = &st.data {
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
                        .insert(name.clone(), ColumnMeta::new(name.clone(), st.ident.to_string()));
                    for field in fields.iter() {
                        let field_name = &field.ident.as_ref().unwrap().to_string();
                        for attr in field.attrs.iter() {
                            let indexes = Indexes::parse_from_attr(attr, field_name)?;
                            if indexes.is_some() {
                                field_index.push(indexes.unwrap().create_index())
                            }
                        }
                    }
                }
            }
        }
    } else {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Only named struct is supported",
        ));
    }

    // 为Server<T>实现CollectionInit，在初始化时创建索引
    let struct_name = &st.ident;
    if drop {
        res.extend(quote! {
            #[axum_mongodb::async_trait]
            impl axum_mongodb::CollectionInit for crate::Server<#struct_name> {
                async fn init(&self) {
                    self.drop_indexes(None).await.unwrap_or_else(|e|{
                        tracing::error!("drop_indexes error: {:?}", e);
                        ()
                    });;

                    let doc_list = vec![#(#field_index),*];
                    for item in doc_list {
                        let mut options = mongodb::options::IndexOptions::default();
                            if item.unique == true {
                                options.unique = Some(true);
                            }
                            if item.name.is_some() {
                                options.name = item.name.clone();
                            }
                            let create_index = self
                                .create_index(
                                    mongodb::IndexModel::builder()
                                        .keys(item.keys)
                                        .options(options)
                                        .build(),
                                    None,
                                )
                                .await;
                            match create_index {
                                Ok(_) => {
                                    tracing::info!("create_index {:#?}", create_index);
                                }
                                Err(e) => {
                                    tracing::error!("create_index error: {:#?}", e);
                                }
                            }
                        }
                }
            }
        });
    } else {
        res.extend(quote! {
            #[axum_mongodb::async_trait]
            impl axum_mongodb::CollectionInit for crate::Server<#struct_name> {
                async fn init(&self) {
                    use axum_mongodb::futures::TryStreamExt;
                    let index = self.list_indexes(None).await;
                    let doc_list:Vec<axum_mongodb::CreateIndexOptions> = vec![#(#field_index),*];
                    match index {
                        Ok(index) => {
                            let index_list = index
                                .try_collect::<Vec<_>>()
                                .await
                                .expect("collect index errored");
                            tracing::info!("{:#?}", index_list);

                            for item_name in &index_list {
                                for doc_item in &doc_list {
                                    if item_name.keys == doc_item.keys {
                                        if item_name.options.is_some() {
                                            self.drop_index(
                                                item_name.options.clone().unwrap().name.as_ref().unwrap(),
                                                None,
                                            )
                                            .await
                                            .expect("drop index error");
                                        }
                                    }
                                }
                            }

                            for item in doc_list {
                                let mut options = mongodb::options::IndexOptions::default();
                                if item.unique == true {
                                    options.unique = Some(true);
                                }
                                if item.name.is_some() {
                                    options.name = item.name.clone();
                                }
                                let create_index = self
                                    .create_index(
                                        mongodb::IndexModel::builder()
                                            .keys(item.keys)
                                            .options(options)
                                            .build(),
                                        None,
                                    )
                                    .await;
                                tracing::info!("create_index {:#?}", create_index);
                            }
                        }
                        Err(e) => {
                            tracing::error!("{:#?}", e);
                        }
                    }
                }
            }
        });
    }
    let name:syn::Ident=syn::parse_str(&name)?;
    res.extend(quote!{
        impl axum::extract::FromRef<axum_mongodb::MongoDbServer<crate::Servers>> for crate::Server<#struct_name> {
            fn from_ref(input: &MongoDbServer<crate::Servers>) -> Self {
                input.servers.#name.clone()
            }
        }
    });

    Ok(res)
}
