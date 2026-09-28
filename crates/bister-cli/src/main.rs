use std::{env, fs, path::PathBuf};

use anyhow::{Context, Result};
use bister_spg::SpgDocument;

fn main() -> Result<()> {
    let mut args = env::args().skip(1);

    match args.next().as_deref() {
        Some("check") => {
            let path = PathBuf::from(
                args.next()
                    .context("usage: bister check <path-to-spg.json>")?,
            );

            let source = fs::read_to_string(&path)
                .with_context(|| format!("failed to read {}", path.display()))?;

            let document: SpgDocument = serde_json::from_str(&source)
                .with_context(|| format!("failed to parse {}", path.display()))?;

            bister_compiler::compile(&document)?;

            println!("✓ {}", path.display());
            Ok(())
        }
        _ => {
            eprintln!("Bister compiler (pre-alpha)");
            eprintln!();
            eprintln!("Usage:");
            eprintln!("  bister check <path-to-spg.json>");
            Ok(())
        }
    }
}
