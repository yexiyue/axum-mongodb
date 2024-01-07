use crate::Server;
use anyhow::Result;
use axum_mongodb::futures::TryStreamExt;
use axum_mongodb::preload::*;
use mongodb::{
    bson::{self, doc, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Column, Deserialize, Serialize, Clone)]
pub struct Todo {
    #[serde(
        serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string",
        rename = "_id"
    )]
    id: ObjectId,
    description: String,
    completed: bool,
    create_time: chrono::DateTime<chrono::Local>,
    update_time: chrono::DateTime<chrono::Local>,
}

impl Server<Todo> {
    pub async fn create_todo(&self, description: String) -> Result<InsertOneResult> {
        Ok(self
            .insert_one(
                Todo {
                    id: ObjectId::new(),
                    description,
                    completed: false,
                    create_time: chrono::Local::now(),
                    update_time: chrono::Local::now(),
                },
                None,
            )
            .await?)
    }

    pub async fn get_todo(&self, id: String) -> Result<Option<Todo>> {
        Ok(self.find_one(doc! {"_id":id}, None).await?)
    }

    pub async fn get_todos(&self) -> Result<Vec<Todo>> {
        let res = self.find(None, None).await?;
        let todos = res.try_collect().await?;
        Ok(todos)
    }

    pub async fn delete_todo(&self, id: String) -> Result<DeleteResult> {
        Ok(self.delete_one(doc! {"_id":id}, None).await?)
    }

    pub async fn update_todo(
        &self,
        id: String,
        description: String,
        completed: Option<bool>,
    ) -> Result<UpdateResult> {
        let mut update = doc! {"$set": {"description": description, "update_time": chrono::Local::now().to_rfc3339()}};
        if let Some(completed) = completed {
            update
                .get_document_mut("$set")
                .unwrap()
                .insert("completed", completed);
        }
        let res = self.update_one(doc! {"_id":id}, update, None).await?;
        Ok(res)
    }
}
