# Bister Technology Stack

Bister intentionally keeps its implementation stack narrow.

## Allowed implementation technologies

### Rust

Rust is the preferred default for new compiler and systems code because it provides:

- memory safety without a garbage collector;
- strong type modeling for compiler data structures;
- good cross-platform support;
- strong tooling and testing;
- practical interoperability with C and C++ libraries.

Likely Rust areas:

- SPG parser and validator;
- Bister IR;
- compiler passes;
- diagnostics;
- CLI;
- target descriptions;
- deterministic build tooling;
- AI provider orchestration where no browser UI is involved.

### C++

C++ is an approved core language when it provides a concrete technical advantage.

Likely reasons include:

- direct use of compiler infrastructure or native SDKs;
- LLVM/Clang integration where C++ APIs are materially simpler;
- performance-critical native components;
- existing high-quality C/C++ libraries that should not be reimplemented.

Bister should avoid duplicating the same subsystem in Rust and C++ without a clear reason.

### Web technologies

Web technologies are allowed for human-facing interfaces, especially the interactive flowchart/semantic editor.

Expected uses include:

- the visual flowchart editor;
- node/property panels;
- semantic graph visualization;
- diagnostics;
- AI proposal review;
- documentation or playground interfaces.

Likely technologies may include TypeScript, JavaScript, HTML, CSS, SVG, Canvas, or WebAssembly as needed.

The browser/editor layer must remain separate from the compiler's semantic truth. UI layout, coordinates, colors, and framework state must never define Bister program semantics.

## Architecture boundary

```text
                Interactive Editor
          TypeScript / Web Technology
                    │
                    │ SPG / compiler API
                    ▼
        ┌──────────────────────────┐
        │      Bister Core         │
        │      Rust / C++          │
        │                          │
        │ SPG • IR • Verification  │
        │ Compiler • CLI • Targets │
        └────────────┬─────────────┘
                     │
                     ▼
              LLVM / WASM / Native
```

## Dependency rule

The compiler core must not require a browser, Electron, Node.js, or a web framework to compile a Bister program.

A command-line or library-based deterministic build must remain possible independently of the visual editor.

## Language policy

For first-party Bister implementation code:

- **Rust:** preferred;
- **C++:** allowed where technically justified;
- **TypeScript/JavaScript/web technologies:** allowed for interactive interfaces and closely related tooling;
- additional implementation languages should not be introduced without an explicit architecture discussion.

This policy applies to the Bister toolchain implementation, not to generated target artifacts or unavoidable third-party dependencies.
