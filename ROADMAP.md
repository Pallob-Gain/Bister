# Bister Roadmap

Bister is an experimental AI-native programming language and compiler architecture. This roadmap is milestone-driven rather than date-driven: each phase has a concrete technical exit condition before the next layer becomes a dependency.

## Project direction

```text
Human intent / algorithm
          ↓
Semantic Program Graph (SPG)
          ↓
Deterministic semantic validation
          ↓
AI-assisted semantic synthesis
          ↓
Bister IR
          ↓
Verification
          ↓
LLVM / WASM / target backend
          ↓
Executable program
```

The project intentionally proves the deterministic compiler core before making AI mandatory.

## Phase 0 — Project foundation

**Status:** In progress

**Goal:** establish shared terminology, architecture, contribution rules, and research boundaries.

Deliverables:

- architecture specification;
- language design principles;
- SPG concept;
- Bister IR concept;
- AI trust boundary;
- verification strategy;
- contribution and security policies;
- example semantic program.

**Exit criteria:** contributors can discuss design changes using shared definitions and can distinguish source semantics, AI synthesis, IR, verification, and backend compilation.

---

## Phase 1 — Minimal deterministic compiler

**Goal:** compile a small semantic program without using an AI model.

Initial semantic features:

- primitive boolean/integer/float values;
- constants and variables;
- arithmetic;
- comparisons;
- branches;
- loops;
- functions;
- explicit inputs and outputs.

Target pipeline:

```text
SPG JSON
   ↓
Parser
   ↓
Validator
   ↓
Bister IR
   ↓
LLVM IR or WASM
   ↓
Executable
```

Primary deliverables:

- SPG schema v0.1;
- parser and validator;
- minimal Bister IR v0.1;
- one backend;
- structured diagnostics;
- golden/compiler tests;
- end-to-end temperature-fan example.

**Exit criteria:** the same accepted SPG produces the same executable behavior without network access or model inference.

---

## Phase 2 — Semantic constraints and effects

**Goal:** move beyond ordinary control flow and begin representing requirements that make Bister different from a conventional AST.

Add:

- physical units;
- ranges;
- assertions;
- mutable/persistent state;
- effects;
- explicit failure paths;
- capability/resource declarations;
- runtime-enforced constraints.

Example:

```text
motor_current <= 8 A
control_period = 1 ms
allocation = static
sensor_timeout > 100 ms -> MOTOR_STOP
```

**Exit criteria:** invalid or unsatisfied constraints produce deterministic diagnostics, and required runtime checks survive lowering.

---

## Phase 3 — AI semantic synthesis

**Goal:** let AI construct or revise formal semantics without making the model the compiler.

Pipeline:

```text
Human requirement
      ↓
Model
      ↓
Structured semantic proposal
      ↓
SPG diff
      ↓
Deterministic validation
      ↓
Developer acceptance
```

Deliverables:

- provider-neutral model interface;
- structured proposal schema;
- local/open-model adapter;
- hosted-model adapter;
- ambiguity/clarification protocol;
- provenance metadata;
- semantic proposal evaluation suite.

**Exit criteria:** a model can translate useful human requirements into valid proposed SPG changes while deterministic validation remains authoritative.

---

## Phase 4 — Visual programming environment

**Goal:** make the semantic program understandable and editable without manually editing graph serialization.

The editor should expose:

- algorithm flow;
- data flow;
- conditions;
- state;
- constraints;
- timing;
- resources;
- failure paths;
- AI-proposed changes;
- compiler diagnostics.

Editor layout must remain separate from semantic meaning.

**Exit criteria:** a developer can construct, inspect, validate, and compile a Phase 2 Bister program entirely through the editor.

---

## Phase 5 — Embedded and real-time target

**Goal:** prove Bister on physical hardware where constraints and resource semantics matter.

Initial hardware concepts:

- GPIO;
- ADC;
- PWM;
- timers;
- interrupts;
- UART;
- I2C;
- SPI;
- static memory;
- task periods and deadlines.

Possible first targets include ESP32, STM32, RP2040, or a small RISC-V MCU. The exact target should be chosen through a design issue.

**Exit criteria:** a Bister semantic program controls real hardware, its resource allocation is inspectable, and its required constraints are either verified or clearly identified as runtime/unverified.

---

## Phase 6 — Strong verification

**Goal:** reduce the gap between intended behavior and executable behavior.

Research areas:

- symbolic execution;
- SMT-based constraint solving;
- abstract interpretation;
- model checking;
- equivalence checking;
- scheduling analysis;
- worst-case execution-time analysis;
- proof-carrying transformations.

**Exit criteria:** selected classes of requirements can be traced from semantic source through IR to a proof, deterministic check, or explicit runtime enforcement.

---

## Phase 7 — Language and ecosystem stabilization

**Goal:** turn successful research concepts into a usable programming ecosystem.

Potential components:

- stable textual representation;
- project manifest;
- module/package system;
- reusable semantic components;
- language server;
- debugger;
- formatter;
- package registry;
- target SDKs;
- editor plugins;
- reproducible model-assisted build records.

**Exit criteria:** a versioned Bister specification and toolchain can support external projects without depending on unstable internal schemas.

---

## Long-term vision

A mature Bister program should allow a developer to describe a system in terms such as:

```text
Three motors have encoders and current sensors.

Maintain commanded velocity using closed-loop control.

Motor current must never exceed 8 A.

If communication is lost for 500 ms,
decelerate all motors into a safe state.

The control loop runs at 1 kHz.

Motor control must continue independently of the UI.
```

Bister should convert that intent into explicit, reviewable semantics; synthesize implementation details where appropriate; verify what can be proven; clearly identify what cannot; and deterministically generate a target executable.

The objective is not to remove programming.

The objective is to move programming closer to **algorithms, behavior, constraints, and intent**.
