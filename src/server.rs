use crate::NewWithDb;
use mongodb::{Collection, Database};
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Server<T>(Collection<T>);

impl<T> Deref for Server<T> {
    type Target = Collection<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> AsRef<Collection<T>> for Server<T> {
    fn as_ref(&self) -> &Collection<T> {
        &self.0
    }
}

impl<T> NewWithDb for Server<T> {
    fn new(db: &Database) -> Self {
        //通过类型名称设置集合
        let type_name = std::any::type_name::<T>();
        let mut collection_name = type_name.split("::").last().unwrap().to_lowercase();
        collection_name.push('s');
        let collection = db.collection::<T>(&collection_name);
        Self(collection)
    }
}
