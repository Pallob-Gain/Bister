//! Bister deterministic compiler pipeline.

use bister_ir::{Module, ModuleError};
use bister_spg::{SpgDocument, ValidationError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompileError {
    #[error(transparent)]
    InvalidSpg(#[from] ValidationError),

    #[error(transparent)]
    InvalidModule(#[from] ModuleError),
}

/// Validate and lower an SPG document into the initial Bister IR.
///
/// The lowering is intentionally minimal while SPG v0.1 and Bister IR v0.1
/// are being specified.
pub fn compile(document: &SpgDocument) -> Result<Module, CompileError> {
    bister_spg::validate(document)?;

    Module::new(document.program.name.clone()).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bister_spg::{Node, Program, SPG_VERSION};

    #[test]
    fn lowers_valid_document_to_module() {
        let document = SpgDocument {
            spg_version: SPG_VERSION.to_string(),
            program: Program {
                name: "TemperatureFanController".to_string(),
                nodes: vec![Node {
                    id: "threshold_60".to_string(),
                    kind: "literal".to_string(),
                }],
            },
        };

        let module = compile(&document).expect("document should compile");

        assert_eq!(module.name(), "TemperatureFanController");
    }

    #[test]
    fn surfaces_spg_validation_errors() {
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

        let error = compile(&document).expect_err("duplicate node ids should fail");

        assert!(matches!(
            error,
            CompileError::InvalidSpg(ValidationError::DuplicateNodeId(ref id)) if id == "n1"
        ));
    }
}
