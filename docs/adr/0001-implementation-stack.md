# ADR 0001: Implementation Stack

- **Status:** Accepted
- **Date:** 2026-09-28

## Context

Bister needs a small, maintainable implementation stack suitable for compiler development, native tooling, embedded targets, and a future interactive flowchart editor.

A broad multi-language codebase would increase build complexity, contributor overhead, FFI boundaries, and long-term maintenance.

## Decision

### Compiler and native tooling

**Rust is the default implementation language.**

The initial compiler workspace contains:

- `bister-spg` — semantic source representation and validation;
- `bister-ir` — deterministic intermediate representation;
- `bister-compiler` — compiler pipeline;
- `bister` — command-line interface.

### C++

C++ may be introduced when it provides a specific technical advantage, especially:

- direct LLVM/Clang APIs;
- native compiler infrastructure that is awkward or incomplete through Rust bindings;
- mature third-party C/C++ libraries;
- performance-sensitive native integration.

A C++ component should have a narrow, documented interface to Rust.

### Web technologies

TypeScript/JavaScript, HTML, CSS, SVG, Canvas, WebAssembly, and suitable web UI frameworks may be used for the interactive flowchart/editor.

The editor is a client of Bister semantics; it does not define them.

### Dependency boundary

The deterministic compiler must remain buildable and usable without:

- a browser;
- Node.js;
- Electron;
- a JavaScript runtime;
- an AI service.

## Consequences

Benefits:

- one primary systems language;
- strong memory safety for compiler structures;
- straightforward CLI and embedded tooling;
- limited FFI surface;
- independent native compiler;
- freedom to build the editor with modern web tooling.

Trade-offs:

- some LLVM integration may eventually require C++ or bindings;
- editor and compiler will use different technology stacks;
- API boundaries between the visual editor and compiler must be explicit.

## Follow-up

If C++ is introduced, a new ADR should define the exact FFI ownership, error, memory, and build boundaries.
