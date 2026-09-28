# Compiler

Bister's active compiler implementation lives in the root Rust workspace under `crates/`.

```text
crates/
├── bister-spg/       Semantic Program Graph types and validation
├── bister-ir/        Deterministic Bister intermediate representation
├── bister-compiler/  Compiler pipeline
└── bister-cli/       `bister` command-line interface
```

This directory is retained as an architectural namespace for future native compiler support files if needed, such as a narrow C++/LLVM bridge.

Rust is the default implementation language. C++ should only be added when a concrete native/compiler-infrastructure advantage justifies the additional FFI boundary.

See [Technology Stack](../docs/technology-stack.md) and [ADR 0001](../docs/adr/0001-implementation-stack.md).
