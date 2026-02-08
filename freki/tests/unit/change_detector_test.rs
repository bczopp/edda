//! Tests für Document-Change-Detector (Phase 7.1.1).

use freki::indexing::{Document, DocumentChangeDetector, DocumentHash};
use serde_json::json;

fn doc(id: &str, content: &str) -> Document {
    Document {
        id: id.to_string(),
        content: content.to_string(),
        metadata: json!({}),
    }
}

#[test]
fn compute_hash_same_document_returns_same_hash() {
    let detector = DocumentChangeDetector;
    let d = doc("id1", "same content");
    let h1 = detector.compute_hash(&d);
    let h2 = detector.compute_hash(&d);
    assert_eq!(h1, h2);
}

#[test]
fn compute_hash_different_content_returns_different_hash() {
    let detector = DocumentChangeDetector;
    let h1 = detector.compute_hash(&doc("id1", "content A"));
    let h2 = detector.compute_hash(&doc("id1", "content B"));
    assert_ne!(h1, h2);
}

#[test]
fn compute_hash_different_id_returns_different_hash() {
    let detector = DocumentChangeDetector;
    let h1 = detector.compute_hash(&doc("id1", "same content"));
    let h2 = detector.compute_hash(&doc("id2", "same content"));
    assert_ne!(h1, h2);
}

#[test]
fn has_changed_false_when_document_unchanged() {
    let detector = DocumentChangeDetector;
    let d = doc("id1", "content");
    let h = detector.compute_hash(&d);
    assert!(!detector.has_changed(&h, &d));
}

#[test]
fn has_changed_true_when_content_changed() {
    let detector = DocumentChangeDetector;
    let d_old = doc("id1", "old content");
    let h = detector.compute_hash(&d_old);
    let d_new = doc("id1", "new content");
    assert!(detector.has_changed(&h, &d_new));
}

#[test]
fn changed_chunk_indices_empty_when_all_unchanged() {
    let detector = DocumentChangeDetector;
    let chunks = vec!["chunk1".to_string(), "chunk2".to_string()];
    let hashes: Vec<DocumentHash> = chunks
        .iter()
        .map(|c| detector.compute_content_hash(c))
        .collect();
    let changed = detector.changed_chunk_indices(&hashes, &chunks);
    assert!(changed.is_empty());
}

#[test]
fn changed_chunk_indices_returns_indices_of_changed_chunks() {
    let detector = DocumentChangeDetector;
    let old_chunks = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let old_hashes: Vec<DocumentHash> = old_chunks
        .iter()
        .map(|c| detector.compute_content_hash(c))
        .collect();
    let new_chunks = vec!["a".to_string(), "b2".to_string(), "c".to_string()];
    let changed = detector.changed_chunk_indices(&old_hashes, &new_chunks);
    assert_eq!(changed, vec![1]);
}

#[test]
fn changed_chunk_indices_new_chunks_longer() {
    let detector = DocumentChangeDetector;
    let old_chunks = vec!["a".to_string()];
    let old_hashes: Vec<DocumentHash> = old_chunks
        .iter()
        .map(|c| detector.compute_content_hash(c))
        .collect();
    let new_chunks = vec!["a".to_string(), "b".to_string()];
    let changed = detector.changed_chunk_indices(&old_hashes, &new_chunks);
    assert_eq!(changed, vec![1]);
}

#[test]
fn compute_content_hash_deterministic() {
    let detector = DocumentChangeDetector;
    let h1 = detector.compute_content_hash("text");
    let h2 = detector.compute_content_hash("text");
    assert_eq!(h1, h2);
}
