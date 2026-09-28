//! Deterministic Bister intermediate representation.
//!
//! Bister IR is the boundary after semantic ambiguity has been resolved
//! enough for conventional compiler lowering.

use thiserror::Error;

/// Stable initial IR module boundary used during Phase 1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    name: String,
}

/// Deterministic module construction errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ModuleError {
    #[error("module name must not be empty")]
    EmptyName,
}

impl Module {
    /// Create a minimal IR module with a stable name boundary.
    pub fn new(name: impl Into<String>) -> Result<Self, ModuleError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(ModuleError::EmptyName);
        }

        Ok(Self { name })
    }

    /// Return the module name selected by the deterministic compiler pipeline.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_non_empty_name() {
        let module = Module::new("controller").expect("module should be created");

        assert_eq!(module.name(), "controller");
    }

    #[test]
    fn rejects_empty_name() {
        assert_eq!(Module::new("   "), Err(ModuleError::EmptyName));
    }
}
