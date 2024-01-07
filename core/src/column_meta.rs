use once_cell::sync::Lazy;
use std::collections::HashMap;
// 全局保存元信息
pub static mut COLUMN_META: Lazy<HashMap<String, ColumnMeta>> = Lazy::new(|| HashMap::new());

// 用于存储列的元数据
#[derive(Debug)]
pub struct ColumnMeta {
    pub name: String,
    pub struct_name: String,
}

impl ColumnMeta {
    pub fn new(name: String, struct_name: String) -> Self {
        Self { name, struct_name }
    }
}

unsafe impl Sync for ColumnMeta {}
unsafe impl Send for ColumnMeta {}
