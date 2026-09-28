# Bister Architecture

## 1. Purpose

Bister is an AI-native programming system in which the maintained program is a semantic description of behavior rather than conventional implementation syntax.

The architecture deliberately separates:

- **human intent**;
- **formal program semantics**;
- **AI-assisted synthesis**;
- **deterministic verification**;
- **target-specific code generation**.

The central design rule is:

> AI may help construct an implementation, but executable meaning must cross a deterministic, inspectable boundary before code generation.

## 2. High-level pipeline

```text
Authoring
   │
   ├─ Visual flow editor
   ├─ Structured textual form
   └─ Natural-language / AI interaction
   │
   ▼
Semantic Program Graph (SPG)
   │
   ├─ type checking
   ├─ control/data-flow analysis
   ├─ constraint normalization
   └─ ambiguity detection
   │
   ▼
AI-assisted semantic lowering
   │
   ▼
Bister IR
   │
   ├─ deterministic validation
   ├─ safety/resource checks
   ├─ optimization
   └─ target legality checks
   │
   ▼
Backend IR
   │
   ├─ LLVM IR
   ├─ WebAssembly
   └─ future backends
   │
   ▼
Executable artifact
```

## 3. Implementation technology boundary

Bister keeps the core toolchain separate from presentation technology.

```text
Interactive Flowchart / Editor
     Web technology
          │
          ▼
   SPG / compiler API
          │
          ▼
Bister Core — Rust / C++
          │
          ▼
 LLVM / WASM / Native
```

Rust is the preferred default for new compiler and systems components. C++ is allowed where it provides a concrete advantage, such as direct compiler infrastructure integration or native libraries. Web technologies are appropriate for the interactive flowchart/editor layer, but they must not define program semantics.

The compiler must remain usable as a native CLI/library without requiring a browser, Node.js, Electron, or a web framework.

See [technology-stack.md](technology-stack.md).

## 4. Front ends

Bister should permit several ways to author the same semantics.

### Visual front end

A graph-based editor can expose:

- operations;
- decisions;
- loops;
- states;
- functions;
- resources;
- constraints;
- timing;
- failure behavior.

### Structured textual front end

A textual representation is useful for:

- version control;
- code review;
- automation;
- hand editing;
- tests.

The initial textual syntax is not yet standardized.

### AI-assisted front end

A developer may describe intent using natural language. The model converts that description into proposed semantic structures.

The model output is never accepted merely because it is syntactically well formed. Missing or ambiguous semantics must be surfaced for review.

## 5. Semantic Program Graph

The SPG is the source-level formal representation.

It describes what the program means using typed nodes and relationships, including:

- data;
- control flow;
- state;
- effects;
- resources;
- constraints;
- timing;
- failures;
- preconditions;
- postconditions.

See [semantic-program-graph.md](semantic-program-graph.md).

## 6. AI reasoning layer

AI is used where conventional deterministic compilation has insufficient information to choose or synthesize an implementation.

Examples:

- expanding high-level behavior into lower-level operations;
- selecting an algorithm from permitted alternatives;
- inferring implementation structure from approved intent;
- proposing target-specific mappings;
- explaining diagnostics.

AI must not silently invent requirements.

Every AI-derived semantic decision should eventually be one of:

1. formally represented and accepted;
2. rejected;
3. marked unresolved and sent back to the developer.

## 7. Bister IR

Bister IR is the deterministic compiler boundary.

By the time a program reaches Bister IR:

- types must be resolved;
- control flow must be explicit;
- state mutation must be explicit;
- effects must be known;
- unresolved natural-language ambiguity must not remain;
- target-relevant constraints must be representable.

See [bister-ir.md](bister-ir.md).

## 8. Verification

Verification occurs at several levels:

```text
SPG structural validation
        ↓
type/effect validation
        ↓
constraint checks
        ↓
Bister IR validation
        ↓
target/resource validation
        ↓
backend verification
        ↓
tests / simulation
```

See [verification.md](verification.md).

## 9. Backend strategy

The first practical backend should use mature compiler infrastructure instead of implementing machine code generation from scratch.

A likely first path is:

```text
Bister IR
  ↓
LLVM IR
  ↓
LLVM optimization/code generation
  ↓
native executable
```

A WebAssembly backend is also attractive because it provides a constrained portable execution target.

## 10. Runtime

Bister should avoid requiring a heavy runtime for all targets.

Runtime services may be optional and target dependent, for example:

- task scheduling;
- garbage collection, if a future memory model needs it;
- async I/O;
- fault handling;
- telemetry;
- reflection.

The initial prototype should favor a minimal runtime.

## 11. Hardware targets

Embedded systems are a useful early target because the semantic language can directly model:

- GPIO;
- timers;
- ADC;
- PWM;
- buses;
- interrupts;
- task periods;
- memory limits;
- safety behavior.

Hardware descriptions should be separate from application intent so that the same semantic program can be remapped where possible.

## 12. Build reproducibility

AI inference is probabilistic, so Bister must distinguish between:

### semantic synthesis

Potentially nondeterministic. Used to produce or revise accepted semantic artifacts.

### deterministic build

Once the accepted SPG/Bister IR and configuration are fixed, downstream compilation should not require fresh nondeterministic model decisions.

This distinction is essential for reproducible builds, review, caching, and certification.

## 13. Long-term architecture principle

Bister should remain usable even if today's LLM architectures are replaced.

The durable asset is not a particular model or prompt. It is the formal semantic representation, validation rules, IR, and compiler contracts.
