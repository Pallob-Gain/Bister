# Bister Rust Workspace

Bister's deterministic core is organized as a Cargo workspace.

## Crates

### `bister-spg`

Owns the Semantic Program Graph data model and deterministic structural validation.

### `bister-ir`

Owns Bister's deterministic compiler intermediate representation.

### `bister-compiler`

Coordinates validation and lowering from SPG into Bister IR and, later, backend IR.

### `bister`

Command-line interface.

## Dependency direction

```text
bister CLI
    │
    ▼
bister-compiler
    │
    ├────► bister-spg
    │
    └────► bister-ir
```

The SPG and IR crates should not depend on UI or AI-provider implementations.
