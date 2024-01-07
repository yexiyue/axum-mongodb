该库是axum-mongodb的核心库，主要提供其中宏相关的实现

提供以下宏

- Column：`#[derive(Column)]`Derive宏，用于收集结构体元信息
- main：`#[axum_mongodb::main]`属性宏，在main函数上使用，主要生成相关结构体，例如Servers、Server
- Inject：`#[axum_mongodb::inject]`属性宏，用于axum handler上，主要作用是替换`DBServers`到`axum_mongodb::MongoDbServer<crate::Servers>`

该库不支持直接使用，具体用法请查看[axum_mongodb](https://docs.rs/axum-mongodb/0.1.3/axum_mongodb/)

