#[cfg(test)]
mod tests {
    use freki::indexing::{BatchIndexingManager, RecordingIndexer};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_batch_indexing_indexes_all_documents() {
        let backend = Arc::new(RecordingIndexer::new());
        let batch = BatchIndexingManager::new(backend.clone(), 2);
        let items = vec![
            (b"doc1".to_vec(), "txt".to_string()),
            (b"doc2".to_vec(), "md".to_string()),
            (b"doc3".to_vec(), "txt".to_string()),
        ];
        let result = batch.index_batch(items).await.unwrap();
        assert_eq!(result.indexed, 3);
        assert_eq!(result.failed, 0);
        assert!(result.errors.is_empty());
        let calls = backend.calls();
        assert_eq!(calls.len(), 3);
        assert_eq!(calls[0].0, b"doc1");
        assert_eq!(calls[0].1, "txt");
        assert_eq!(calls[1].0, b"doc2");
        assert_eq!(calls[1].1, "md");
        assert_eq!(calls[2].0, b"doc3");
        assert_eq!(calls[2].1, "txt");
    }

    #[tokio::test]
    async fn test_batch_indexing_respects_batch_size() {
        let backend = Arc::new(RecordingIndexer::new());
        let batch = BatchIndexingManager::new(backend.clone(), 2);
        let items = vec![
            (b"a".to_vec(), "txt".to_string()),
            (b"b".to_vec(), "txt".to_string()),
            (b"c".to_vec(), "txt".to_string()),
        ];
        let _ = batch.index_batch(items).await.unwrap();
        let calls = backend.calls();
        assert_eq!(calls.len(), 3);
    }

    #[tokio::test]
    async fn test_batch_indexing_tracks_failures() {
        let backend = Arc::new(RecordingIndexer::new());
        backend.set_fail_at(Some(1));
        let batch = BatchIndexingManager::new(backend.clone(), 2);
        let items = vec![
            (b"ok".to_vec(), "txt".to_string()),
            (b"fail".to_vec(), "txt".to_string()),
            (b"ok2".to_vec(), "txt".to_string()),
        ];
        let result = batch.index_batch(items).await.unwrap();
        assert_eq!(result.indexed, 2);
        assert_eq!(result.failed, 1);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0].0, 1);
        assert!(result.errors[0].1.contains("mock fail"));
    }

    #[tokio::test]
    async fn test_batch_indexing_progress_callback() {
        let backend = Arc::new(RecordingIndexer::new());
        let progress_calls = Arc::new(std::sync::Mutex::new(Vec::<(usize, usize)>::new()));
        let progress_calls_clone = Arc::clone(&progress_calls);
        let batch = BatchIndexingManager::new(backend, 2).with_progress(move |current, total| {
            progress_calls_clone.lock().unwrap().push((current, total));
        });
        let items = vec![
            (b"a".to_vec(), "txt".to_string()),
            (b"b".to_vec(), "txt".to_string()),
        ];
        let _ = batch.index_batch(items).await.unwrap();
        let calls = progress_calls.lock().unwrap();
        assert!(!calls.is_empty());
        assert_eq!(calls.last(), Some(&(2, 2)));
    }

    #[tokio::test]
    async fn test_batch_indexing_empty_items() {
        let backend = Arc::new(RecordingIndexer::new());
        let batch = BatchIndexingManager::new(backend.clone(), 2);
        let result = batch.index_batch(vec![]).await.unwrap();
        assert_eq!(result.indexed, 0);
        assert_eq!(result.failed, 0);
        assert!(backend.calls().is_empty());
    }
}
