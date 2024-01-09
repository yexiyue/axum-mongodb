use crate::NewWithDb;
use axum::{async_trait, extract::FromRequestParts};
use mongodb::Database;
use std::convert::Infallible;

/**

MongoDbServer
一个结构体，用于存储数据库和集合，一个axum State，为其实现了FromRequestParts，
从而可以通过MongodbServer extract获取数据库和集合
可以使用[`crate::inject`]以及[`crate::preload::DBServers`]简化extract的使用

*/
#[derive(Debug, Clone)]
pub struct MongoDbServer<T>
where
    T: Clone,
{
    pub db: Database,
    pub servers: T,
}

#[async_trait]
impl<T> NewWithDb for MongoDbServer<T>
where
    T: Clone + NewWithDb,
{
    async fn new(db: Database) -> Self {
        Self {
            servers: T::new(db.clone()).await,
            db,
        }
    }
}

#[async_trait]
impl<S, T> FromRequestParts<S> for MongoDbServer<T>
where
    S: Send + Sync,
    T: Clone + Send + Sync + 'static,
{
    type Rejection = Infallible;
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let dbs = parts
            .extensions
            .get::<Self>()
            .expect("can not get MongoDbServer");

        Ok(dbs.clone())
    }
}
