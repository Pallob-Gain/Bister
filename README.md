# Bister

<p align="center">
  <img src="assets/bister-logo.svg" alt="Bister logo — semantic flow transformed into machine representation" width="360" />
</p>

<p align="center"><strong>From human algorithms to machine execution.</strong></p>

**An AI-native programming language for expressing algorithms, intent, and constraints instead of implementation syntax.**

> **Write the logic. Let the machine write the code.**

Bister is an experimental open-source programming language and compiler architecture that explores a different interface between humans and computers.

Today, AI coding tools usually translate human intent into a conventional language such as C, C++, Rust, Python, or JavaScript, after which a normal compiler or runtime takes over. Bister asks a different question:

> If a machine can already reason about a programmer's intent, can the programmer work primarily with algorithms, semantic relationships, constraints, and system behavior rather than implementation syntax?

Bister treats the **program's semantics as the source of truth**. A Bister program may be authored visually, textually, or through AI-assisted interaction, but all front ends must resolve to a formal semantic representation before executable code is produced.

## The idea

```text
Traditional development

Human idea
   ↓
C / C++ / Rust / Python / ...
   ↓
Compiler / runtime
   ↓
Machine execution


Bister

Human intent + algorithm + constraints
   ↓
Semantic Program Graph (SPG)
   ↓
AI-assisted semantic lowering
   ↓
Bister IR
   ↓
Verification
   ↓
Deterministic backend
   ↓
Machine execution
```

Bister does **not** ask an LLM to emit arbitrary executable bytes. AI is a reasoning component inside the toolchain, while the final lowering and code generation are deterministic and inspectable.

## Core principles

1. **Semantics before syntax** — program meaning matters more than how it is entered.
2. **AI-assisted, not AI-trusted** — AI may infer and propose; deterministic validation decides what can be compiled.
3. **Model independent** — Bister should work with local or hosted models through replaceable adapters.
4. **Target independent** — the same semantic program should be portable across suitable targets where practical.
5. **Inspectable** — developers should be able to inspect the semantic graph, IR, generated implementation choices, and diagnostics.
6. **Explicit constraints** — timing, safety, resource, concurrency, and failure requirements should become first-class program semantics.
7. **Reproducible builds** — finalized semantic input and compiler configuration should produce deterministic downstream artifacts.

## Example concept

Instead of manually implementing a motor controller in a conventional language, a Bister program could express:

```text
system MotorController

inputs:
    target_speed: RPM
    measured_speed: RPM
    motor_current: Ampere

behavior:
    maintain measured_speed near target_speed

constraints:
    motor_current <= 8 A
    acceleration <= 300 RPM/s

timing:
    control_loop = 1 kHz

failures:
    motor_current > 8 A      -> MOTOR_STOP
    sensor_timeout > 100 ms  -> MOTOR_STOP
```

The exact surface syntax above is **illustrative, not standardized yet**. Bister's first specification work is focused on the semantics underneath the syntax.

## Proposed architecture

```text
              Authoring Interfaces
      ┌────────────┼────────────┐
      ▼            ▼            ▼
 Visual Flow   Text Format   AI Interaction
      └────────────┼────────────┘
                   ▼
        Semantic Program Graph
                   │
       ┌───────────┴───────────┐
       ▼                       ▼
 AI Reasoner             Static Analyzer
       └───────────┬───────────┘
                   ▼
                Bister IR
                   │
                   ▼
             Verification
                   │
        ┌──────────┼──────────┐
        ▼          ▼          ▼
      LLVM        WASM      Other IR
        │
        ▼
 ARM / x86-64 / RISC-V / target-specific executable
```

## What Bister is not

Bister is not intended to be:

- a prompt-to-C wrapper;
- an LLM that directly emits arbitrary machine code;
- merely a visual scripting editor;
- a no-code platform;
- tied to one AI provider;
- a replacement for deterministic compilers and verification.

The research goal is to discover a rigorous programming representation in which **human intent and algorithmic structure are primary**, while implementation details can be synthesized, checked, explained, and lowered by the toolchain.

## Initial scope

The first prototype should remain intentionally small:

- primitive values and types;
- variables and constants;
- arithmetic and comparisons;
- branches and loops;
- functions;
- state;
- inputs and outputs;
- explicit constraints;
- a machine-readable Semantic Program Graph;
- Bister IR;
- one deterministic backend;
- an optional model adapter;
- validation and diagnostics.

Embedded/control software is a strong early proving ground because timing, state, resources, safety constraints, and hardware interaction can be modeled explicitly.

## Repository map

```text
Bister/
├── README.md
├── LICENSE
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── SECURITY.md
├── ROADMAP.md
├── assets/
│   └── bister-logo.svg
├── docs/
│   ├── architecture.md
│   ├── language-design.md
│   ├── semantic-program-graph.md
│   ├── bister-ir.md
│   ├── ai-integration.md
│   ├── verification.md
│   └── roadmap.md
├── compiler/
├── ai/
├── editor/
├── runtime/
├── targets/
├── examples/
└── tests/
```

## Documentation

Start here:

- [Architecture](docs/architecture.md)
- [Language design](docs/language-design.md)
- [Semantic Program Graph](docs/semantic-program-graph.md)
- [Bister IR](docs/bister-ir.md)
- [AI integration](docs/ai-integration.md)
- [Verification strategy](docs/verification.md)
- [Project roadmap](ROADMAP.md)
- [Detailed roadmap notes](docs/roadmap.md)
- [Contributing](CONTRIBUTING.md)

## Project status

**Status: concept / early research / pre-alpha**

No Bister syntax, IR version, compiler API, or execution model should be considered stable yet. Early contributions should favor explicit design proposals, prototypes, tests, and measurable trade-offs over premature standardization.

## Roadmap at a glance

| Phase | Focus | Exit condition |
| --- | --- | --- |
| **0 — Foundation** | Architecture, terminology, SPG/IR concepts, contribution process | Shared design vocabulary and project boundaries |
| **1 — Deterministic core** | SPG → validator → Bister IR → backend | Small programs compile without AI or network access |
| **2 — Constraints & effects** | Units, state, failures, resources, runtime checks | Requirements survive lowering and invalid semantics are rejected |
| **3 — AI synthesis** | Natural language → structured semantic proposals | AI proposes; deterministic validation remains authoritative |
| **4 — Visual editor** | Graph-based authoring, diagnostics, semantic review | A program can be built without editing SPG JSON |
| **5 — Embedded target** | GPIO, ADC, PWM, timers, buses, deadlines | Bister controls real hardware with inspectable resource mapping |
| **6 — Verification** | SMT, symbolic execution, timing/resource analysis | Selected requirements are provable or explicitly enforced |
| **7 — Stabilization** | Specification, modules, tooling, ecosystem | External projects can target a versioned Bister toolchain |

See the full [Bister roadmap](ROADMAP.md).


## Research questions

Bister is intended to explore questions including:

- What is the minimum formal information needed to describe a program reliably?
- What belongs in the semantic source representation versus generated implementation?
- Which decisions may be delegated to AI, and which must remain deterministic?
- How should uncertainty and ambiguity be represented?
- When should the compiler ask for clarification instead of assuming intent?
- How can an AI-generated implementation be checked against the original specification?
- Can timing, memory, power, hardware resources, and safety constraints become first-class program concepts?
- How should one semantic program target very different machines?
- Can the compiler explain why a particular implementation was selected?

## Contributing

Bister is intended to be community-driven. Compiler engineers, programming-language researchers, embedded developers, AI/ML engineers, verification researchers, IDE developers, and curious experimenters are all welcome.

See [CONTRIBUTING.md](CONTRIBUTING.md) before opening a design PR. Major language changes should begin as a design proposal so that the project can debate semantics before implementation.

## License

Bister is licensed under the [Apache License 2.0](LICENSE).

---

**Bister — from human algorithms to machine execution.**
