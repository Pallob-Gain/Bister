//! Semantic Program Graph (SPG) types and validation.
//!
//! This crate owns Bister's source-level semantic representation.
//! It must remain independent of any specific AI provider or user interface.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Version identifier for the draft SPG schema.
pub const SPG_VERSION: &str = "0.1-draft";

/// Top-level SPG document.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpgDocument {
    pub spg_version: String,
    pub program: Program,
}

/// Minimal program container used during Phase 1.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Program {
    pub name: String,
    #[serde(default)]
    pub nodes: Vec<Node>,
}

/// Minimal semantic node representation.
///
/// This is intentionally small while the v0.1 schema is being designed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Node {
    pub id: String,
    pub kind: String,
}

/// Deterministic SPG validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ValidationError {
    #[error("unsupported SPG version: {0}")]
    UnsupportedVersion(String),

    #[error("program name must not be empty")]
    EmptyProgramName,

    #[error("node id must not be empty")]
    EmptyNodeId,

    #[error("duplicate node id: {0}")]
    DuplicateNodeId(String),
}

/// Validate structural invariants that do not require model inference.
pub fn validate(document: &SpgDocument) -> Result<(), ValidationError> {
    if document.spg_version != SPG_VERSION {
        return Err(ValidationError::UnsupportedVersion(
            document.spg_version.clone(),
        ));
    }

    if document.program.name.trim().is_empty() {
        return Err(ValidationError::EmptyProgramName);
    }

    let mut ids = std::collections::HashSet::new();

    for node in &document.program.nodes {
        if node.id.trim().is_empty() {
            return Err(ValidationError::EmptyNodeId);
        }

        if !ids.insert(node.id.clone()) {
            return Err(ValidationError::DuplicateNodeId(node.id.clone()));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_minimal_program() {
        let document = SpgDocument {
            spg_version: SPG_VERSION.to_string(),
            program: Program {
                name: "Example".to_string(),
                nodes: vec![],
            },
        };

        assert_eq!(validate(&document), Ok(()));
    }

    #[test]
    fn rejects_duplicate_node_ids() {
        let document = SpgDocument {
            spg_version: SPG_VERSION.to_string(),
            program: Program {
                name: "Example".to_string(),
                nodes: vec![
                    Node {
                        id: "n1".to_string(),
                        kind: "literal".to_string(),
                    },
                    Node {
                        id: "n1".to_string(),
                        kind: "literal".to_string(),
                    },
                ],
            },
        };

        assert_eq!(
            validate(&document),
            Err(ValidationError::DuplicateNodeId("n1".to_string()))
        );
    }
}
