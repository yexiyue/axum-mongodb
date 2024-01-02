use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
};
use std::{convert::Infallible, ops::Deref};

#[derive(Debug, Clone)]
pub struct ServersWarp<T>
where
    T: Clone,
{
    inner: T,
}

impl<T> Deref for ServersWarp<T>
where
    T: Clone,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> AsRef<T> for ServersWarp<T>
where
    T: Clone,
{
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T> ServersWarp<T>
where
    T: Clone,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl<S, T> FromRequestParts<S> for ServersWarp<T>
where
    S: Send + Sync,
    Self: FromRef<S>,
    T: Clone,
{
    type Rejection = Infallible;
    async fn from_request_parts(
        _parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}
