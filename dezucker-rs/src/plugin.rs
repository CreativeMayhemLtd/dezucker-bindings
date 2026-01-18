use crate::storage::{BoxResult, InternalStorage};
use crate::types::{FormattedPost, IdField, PostFragment};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Metadata identifying a plugin.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PluginMetadata {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub version: Option<String>,
}

/// Context provided to the OutputSink during persistence.
pub struct ExportContext {
    pub post_id: Option<IdField>,
    pub timestamp: i64,
    pub index: usize,
    pub total: usize,
    pub directory: Option<String>,
    pub media: Option<Vec<PostFragment>>,
    pub storage: Box<dyn InternalStorage>,
    pub extra: HashMap<String, Value>,
}

/// Interface for transforming post data into a target format.
#[async_trait]
pub trait DataTransformer<T, C>: Send + Sync {
    async fn transform(
        &self,
        post: &FormattedPost,
        context: &ExportContext,
        config: Option<&C>,
    ) -> BoxResult<T>;
}

/// Interface for persisting transformed data to a destination.
#[async_trait]
pub trait OutputSink<T, C>: Send + Sync {
    async fn persist(
        &self,
        data: T,
        context: &ExportContext,
        config: Option<&C>,
    ) -> BoxResult<()>;
    fn database_collection_key(&self) -> Option<String> {
        None
    }
}

/// A complete Dezucker plugin definition.
pub trait DezuckerPlugin<T, C>: Send + Sync {
    fn metadata(&self) -> &PluginMetadata;
    fn transformer(&self) -> &dyn DataTransformer<T, C>;
    fn sink(&self) -> &dyn OutputSink<T, C>;
    fn default_config(&self) -> Option<&C> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::BoxResult;
    use crate::types::FormattedPost;
    use async_trait::async_trait;
    use serde_json::json;

    // Mock storage for testing
    struct MockStorage;
    #[async_trait]
    impl InternalStorage for MockStorage {
        async fn init(&self) -> BoxResult<()> { Ok(()) }
        async fn push(&self, _key: &str, _data: Value) -> BoxResult<()> { Ok(()) }
        async fn update(&self, _key: &str, _update_fn: Box<dyn FnOnce(&mut Vec<Value>) + Send>) -> BoxResult<()> { Ok(()) }
        async fn data_for(&self, _key: &str) -> BoxResult<Vec<Value>> { Ok(vec![]) }
        fn collection_keys(&self) -> HashMap<String, String> { HashMap::new() }
    }

    struct RawJsonTransformer;
    #[async_trait]
    impl DataTransformer<Value, ()> for RawJsonTransformer {
        async fn transform(&self, post: &FormattedPost, _context: &ExportContext, _config: Option<&()>) -> BoxResult<Value> {
            Ok(post.raw.clone().unwrap_or(json!({ "text": post.text })))
        }
    }

    struct ConsoleSink;
    #[async_trait]
    impl OutputSink<Value, ()> for ConsoleSink {
        async fn persist(&self, data: Value, context: &ExportContext, _config: Option<&()>) -> BoxResult<()> {
            println!("[Plugin Export] Post {}/{} (ID: {:?})", context.index + 1, context.total, context.post_id);
            println!("{}", serde_json::to_string_pretty(&data)?);
            Ok(())
        }
    }

    struct ConsolePlugin {
        metadata: PluginMetadata,
        transformer: RawJsonTransformer,
        sink: ConsoleSink,
    }

    impl DezuckerPlugin<Value, ()> for ConsolePlugin {
        fn metadata(&self) -> &PluginMetadata { &self.metadata }
        fn transformer(&self) -> &dyn DataTransformer<Value, ()> { &self.transformer }
        fn sink(&self) -> &dyn OutputSink<Value, ()> { &self.sink }
    }

    #[tokio::test]
    async fn test_mock_plugin_execution() {
        let plugin = ConsolePlugin {
            metadata: PluginMetadata {
                name: "Console Export".to_string(),
                slug: "console-export".to_string(),
                description: None,
                version: None,
            },
            transformer: RawJsonTransformer,
            sink: ConsoleSink,
        };

        let post = FormattedPost {
            tid: None,
            id: None,
            text: "Test post".to_string(),
            timestamp: None,
            attachments_count: None,
            meaningful_entries_count: 0,
            tags: None,
            fragments: None,
            media: None,
            raw: Some(json!({"foo": "bar"})),
        };

        let context = ExportContext {
            post_id: None,
            timestamp: 0,
            index: 0,
            total: 1,
            directory: None,
            media: None,
            storage: Box::new(MockStorage),
            extra: HashMap::new(),
        };

        let transformed = plugin.transformer().transform(&post, &context, None).await.unwrap();
        assert_eq!(transformed, json!({"foo": "bar"}));

        plugin.sink().persist(transformed, &context, None).await.unwrap();
    }
}
