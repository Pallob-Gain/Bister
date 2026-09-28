# Bister Roadmap

Bister is pre-alpha. This roadmap describes research milestones rather than release promises.

## Phase 0 — Foundation

**Goal:** establish the design vocabulary.

- project vision;
- architecture;
- language principles;
- SPG draft;
- IR draft;
- AI trust boundary;
- verification model;
- contribution process.

Exit condition: contributors can discuss architecture using shared definitions.

## Phase 1 — Deterministic semantic core

**Goal:** prove the non-AI compiler path first.

Implement a minimal SPG supporting:

- constants;
- primitive numeric/boolean types;
- variables;
- arithmetic;
- comparisons;
- branches;
- loops;
- functions;
- inputs/outputs.

Build:

```text
SPG JSON
  ↓
validator
  ↓
Bister IR
  ↓
LLVM IR or WASM
  ↓
executable
```

No LLM is required for this phase.

Exit condition: a small semantic graph can compile and run deterministically.

## Phase 2 — Constraints and effects

Add:

- units;
- ranges;
- assertions;
- effects;
- failure edges;
- explicit state;
- compiler diagnostics.

Exit condition: invalid semantic programs produce useful deterministic errors.

## Phase 3 — AI semantic authoring

Add provider abstraction and one open model integration.

Prototype:

```text
natural-language requirement
  ↓
AI proposal
  ↓
SPG diff
  ↓
validation
  ↓
developer approval
```

Exit condition: AI can produce useful structured semantic proposals without being trusted as the verifier.

## Phase 4 — Visual editor

Create an editor for:

- graph construction;
- semantic properties;
- constraints;
- diagnostics;
- SPG diff/review;
- AI suggestions.

Exit condition: a developer can build a small Bister program without editing JSON.

## Phase 5 — Embedded prototype

Add an initial hardware model:

- GPIO;
- ADC;
- PWM;
- timers;
- UART/I2C/SPI;
- interrupts;
- static memory constraints.

Choose one initial board/MCU target.

Exit condition: Bister builds and runs a real hardware-control example.

## Phase 6 — Verification expansion

Explore:

- symbolic constraints;
- scheduling/timing analysis;
- resource proofs;
- generated property tests;
- equivalence checks.

## Phase 7 — Ecosystem experiments

Only after the semantic core stabilizes:

- package/module model;
- reusable semantic components;
- debugger;
- language server;
- target SDKs;
- plugin system;
- project manifest;
- reproducible model-assisted build records.

## First recommended implementation issues

1. Define SPG JSON schema v0.1.
2. Choose implementation language for the compiler prototype.
3. Implement SPG parser/validator.
4. Define integer/boolean Bister IR.
5. Lower arithmetic and branches.
6. Add one backend.
7. Add golden tests.
8. Build a temperature-controlled fan example.
9. Define provider-neutral AI proposal schema.
10. Prototype a graph viewer.

## Explicit non-goals for now

- self-hosting;
- production safety certification;
- full natural-language programming;
- arbitrary direct binary generation by a model;
- competing with mature general-purpose languages on feature count.
