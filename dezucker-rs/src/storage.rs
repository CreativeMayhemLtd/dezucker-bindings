use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

pub type BoxResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

#[async_trait]
pub trait InternalStorage: Send + Sync {
    async fn init(&self) -> BoxResult<()>;
    async fn push(&self, key: &str, data: Value) -> BoxResult<()>;
    async fn update(&self, key: &str, update_fn: Box<dyn FnOnce(&mut Vec<Value>) + Send>) -> BoxResult<()>;
    async fn data_for(&self, key: &str) -> BoxResult<Vec<Value>>;
    fn collection_keys(&self) -> HashMap<String, String>;
}
