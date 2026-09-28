//! Bister deterministic compiler pipeline.

use bister_ir::Module;
use bister_spg::{SpgDocument, ValidationError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompileError {
    #[error(transparent)]
    InvalidSpg(#[from] ValidationError),
}

/// Validate and lower an SPG document into the initial Bister IR.
///
/// The lowering is intentionally minimal while SPG v0.1 and Bister IR v0.1
/// are being specified.
pub fn compile(document: &SpgDocument) -> Result<Module, CompileError> {
    bister_spg::validate(document)?;

    Ok(Module {
        name: document.program.name.clone(),
    })
}
