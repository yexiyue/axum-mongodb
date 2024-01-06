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

        impl axum::extract::FromRef<axum_mongodb::MongoDbServer<Servers>> for crate::Server<User> {
            fn from_ref(input: &MongoDbServer<Servers>) -> Self {
                input.servers.users.clone()
            }
        }

        #[async_trait]
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

        #[async_trait]
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
