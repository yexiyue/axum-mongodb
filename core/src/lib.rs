/*!
该库是axum-mongodb的核心库，主要提供其中宏相关的实现

提供以下宏

- [`Column`]：`#[derive(Column)]`Derive宏，用于收集结构体元信息

- [`macro@main`]：`#[axum_mongodb::main]`属性宏，在main函数上使用，主要生成相关结构体，例如Servers、Server

- [`macro@inject`]：`#[axum_mongodb::inject]`属性宏，用于axum handler上，主要作用是替换`DBServers`到`axum_mongodb::MongoDbServer<crate::Servers>`

该库不支持直接使用，具体用法请查看[axum_mongodb](https://docs.rs/axum-mongodb/1.0.1/axum_mongodb/)

*/

#[doc(hidden)]
use proc_macro::TokenStream;
mod column;
pub(crate) mod column_meta;
use column::collect_meta;
use column_meta::COLUMN_META;

#[doc(hidden)]
use quote::quote;

#[doc(hidden)]
use syn::parse_macro_input;
pub(crate) mod indexes;

/**
Column Derive宏，用于收集结构体元信息，以及初始化mongodb的索引

每次修改索引时，默认不删除全部索引，但会尝试删除老的索引

属性列表

- dropIndexes：是否删除当前集合的全部索引，默认不删除

- singleIndex：[单索引](https://www.mongodb.com/docs/manual/core/indexes/index-types/index-single/)

- compoundIndex：[复合索引](https://www.mongodb.com/docs/manual/core/indexes/index-types/index-compound/)

- multikeyIndex：[多键索引](https://www.mongodb.com/docs/manual/core/indexes/index-types/index-multikey/)


# Example
```rust
#[derive(Debug, Clone, Column)]
#[dropIndexes]
struct User {
    #[singleIndex(unique)]
    name: String,
    #[compoundIndex(unique, other_fields(name))]
    age: i32,
    #[multikeyIndex(unique, field_name = "age")]
    address: String,
}
```
 */
#[proc_macro_derive(
    Column,
    attributes(dropIndexes, singleIndex, compoundIndex, multikeyIndex)
)]
pub fn column_derive(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    let drop_indexes = st
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("dropIndexes"));
    let mut drop = false;
    if drop_indexes.is_some() {
        drop = true;
    }
    collect_meta(&st, drop)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/**
最主要的宏，为Server<T>实现[`axum::extract::FromRequestParts`]等
# Example
```rust
#[tokio::main]
#[axum_mongodb::main]
async fn main() {
    //...
}
```
*/
#[proc_macro_attribute]
pub fn main(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let st = parse_macro_input!(input as syn::ItemFn);
    let mut res = proc_macro2::TokenStream::new();
    res.extend(quote!(
        #[derive(Debug, Clone)]
        pub struct Server<T>(mongodb::Collection<T>);

        unsafe impl<T> Send for Server<T> {}
        unsafe impl<T> Sync for Server<T> {}

        impl<T> std::ops::Deref for Server<T> {
            type Target = mongodb::Collection<T>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T> AsRef<mongodb::Collection<T>> for Server<T> {
            fn as_ref(&self) -> &mongodb::Collection<T> {
                &self.0
            }
        }

        #[axum_mongodb::async_trait]
        impl<S, T> axum::extract::FromRequestParts<S> for crate::Server<T>
        where
            S: Send + Sync,
            Self: axum::extract::FromRef<S>,
            T: Clone,
        {
            type Rejection = std::convert::Infallible;
            async fn from_request_parts(
                _parts: &mut axum::http::request::Parts,
                state: &S,
            ) -> Result<Self, Self::Rejection> {
                use axum::extract::FromRef;
                Ok(Self::from_ref(state))
            }
        }

        #[axum_mongodb::async_trait]
        impl<T> axum_mongodb::NewWithDb for Server<T>
        where
            Self: axum_mongodb::CollectionInit,
        {
            async fn new(db: mongodb::Database) -> Self {
                //通过类型名称设置集合
                let type_name = std::any::type_name::<T>();
                let mut collection_name = type_name.split("::").last().unwrap().to_lowercase();
                collection_name.push('s');
                let collection = db.collection::<T>(&collection_name);
                let res = Self(collection);
                res.init().await;
                res
            }
        }

        #[axum_mongodb::inject_meta]
        pub struct Servers {}

        impl axum::extract::FromRef<axum_mongodb::MongoDbServer<crate::Servers>> for crate::Servers
        {
            fn from_ref(input: &axum_mongodb::MongoDbServer<crate::Servers>) -> Self {
                input.servers.clone()
            }
        }

        #[axum_mongodb::async_trait]
        impl<S> axum::extract::FromRequestParts<S> for crate::Servers
        where
            S: Send + Sync,
            Self: axum::extract::FromRef<S>,
        {
            type Rejection = std::convert::Infallible;
            async fn from_request_parts(
                _parts: &mut axum::http::request::Parts,
                state: &S,
            ) -> Result<Self, Self::Rejection> {
                use axum::extract::FromRef;
                Ok(Self::from_ref(state))
            }
        }
        #st
    ));
    res.into()
}

/// 用于宏内部，将收集到的元信息注入到结构体中
#[proc_macro_attribute]
pub fn inject_meta(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let st = parse_macro_input!(input as syn::ItemStruct);
    column::inject_meta(&st)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/**
用于axum handler，用于替换extract类型，简化操作
# Example
```rust
#[axum_mongodb::inject]
async fn db_test(servers: DBServers) -> impl IntoResponse {
    let db_name = servers.db.name();
    Json(json!({
        "db_name":db_name
    }))
}
```
*/
#[proc_macro_attribute]
pub fn inject(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let st = parse_macro_input!(input as syn::ItemFn);
    column::inject(&st)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
