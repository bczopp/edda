//! Document-Change-Detection (Phase 7.1.1): Dokument-Hash, Änderungserkennung, geänderte Teile.

use crate::indexing::Document;
use sha2::{Digest, Sha256};
use std::fmt;

/// Stabler Hash eines Dokuments oder Chunks (SHA-256, 32 Bytes).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct DocumentHash(pub [u8; 32]);

impl fmt::Debug for DocumentHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DocumentHash([..{} bytes..])", self.0.len())
    }
}

/// Berechnet Dokument-Hash, erkennt Änderungen und identifiziert geänderte Chunks.
#[derive(Debug, Clone, Copy, Default)]
pub struct DocumentChangeDetector;

impl DocumentChangeDetector {
    /// Berechnet einen stabilen Hash für das gesamte Dokument (id + content + metadata).
    pub fn compute_hash(&self, document: &Document) -> DocumentHash {
        let mut hasher = Sha256::new();
        hasher.update(document.id.as_bytes());
        hasher.update(b"\0");
        hasher.update(document.content.as_bytes());
        hasher.update(b"\0");
        hasher.update(serde_json::to_string(&document.metadata).unwrap_or_default().as_bytes());
        let digest = hasher.finalize();
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&digest);
        DocumentHash(arr)
    }

    /// Berechnet einen stabilen Hash für einen einzelnen Content-String (z. B. Chunk).
    pub fn compute_content_hash(&self, content: &str) -> DocumentHash {
        let digest = Sha256::digest(content.as_bytes());
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&digest);
        DocumentHash(arr)
    }

    /// Liefert true, wenn sich das Dokument gegenüber dem vorherigen Hash geändert hat.
    pub fn has_changed(&self, previous_hash: &DocumentHash, document: &Document) -> bool {
        self.compute_hash(document) != *previous_hash
    }

    /// Liefert die Indizes der Chunks, die sich geändert haben oder neu sind.
    /// `old_chunk_hashes`: Hashes der bisherigen Chunks (in Reihenfolge).
    /// `new_chunks`: aktuelle Chunk-Texte.
    pub fn changed_chunk_indices(
        &self,
        old_chunk_hashes: &[DocumentHash],
        new_chunks: &[String],
    ) -> Vec<usize> {
        let mut changed = Vec::new();
        for (i, chunk) in new_chunks.iter().enumerate() {
            let new_hash = self.compute_content_hash(chunk);
            let same = old_chunk_hashes.get(i).map(|h| *h == new_hash).unwrap_or(false);
            if !same {
                changed.push(i);
            }
        }
        changed
    }
}
