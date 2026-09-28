//! Deterministic Bister intermediate representation.
//!
//! Bister IR is the boundary after semantic ambiguity has been resolved
//! enough for conventional compiler lowering.

/// Placeholder IR module identifier used during Phase 1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub name: String,
}
