//! Performance-Tests für Parallel-Indexing (Phase 14.1.1).

#[cfg(test)]
mod tests {
    use freki::indexing::{BatchIndexingManager, SingleDocumentIndexer};
    use std::sync::Arc;
    use std::time::{Duration, Instant};
    use async_trait::async_trait;

    /// Mock-Indexer mit konfigurierbarer Verzögerung pro Dokument (simuliert I/O/Embedding).
    struct DelayedIndexer {
        delay_ms: u64,
    }

    #[async_trait]
    impl SingleDocumentIndexer for DelayedIndexer {
        async fn index_bytes(
            &self,
            _bytes: &[u8],
            _file_extension: &str,
        ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            tokio::time::sleep(Duration::from_millis(self.delay_ms)).await;
            Ok(())
        }
    }

    /// Parallel-Indexing muss schneller sein als sequenzielle Summe: N Dokumente mit je D ms
    /// werden in einem Batch parallel verarbeitet → Gesamtzeit deutlich unter N*D.
    #[tokio::test]
    async fn parallel_indexing_faster_than_sequential() {
        const DELAY_MS: u64 = 40;
        const N: usize = 3;
        let sequential_estimate_ms = DELAY_MS * N as u64; // 120 ms wenn sequenziell

        let backend = Arc::new(DelayedIndexer { delay_ms: DELAY_MS });
        let batch = BatchIndexingManager::new(backend, N);
        let items: Vec<(Vec<u8>, String)> = (0..N)
            .map(|i| (format!("doc{}", i).into_bytes(), "txt".to_string()))
            .collect();

        let start = Instant::now();
        let result = batch.index_batch(items).await.unwrap();
        let elapsed = start.elapsed();

        assert_eq!(result.indexed, N as u32);
        assert!(result.failed == 0);
        assert!(
            elapsed.as_millis() < sequential_estimate_ms as u128,
            "parallel batch should complete in under {}ms (sequential estimate), took {}ms",
            sequential_estimate_ms,
            elapsed.as_millis()
        );
    }
}
