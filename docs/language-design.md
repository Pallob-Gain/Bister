# Bister Language Design

## Goal

Bister is designed around **executable intent**.

A programmer should be able to define:

- what data exists;
- what operations are allowed;
- how behavior flows;
- what must always be true;
- what must happen on failure;
- what timing/resource requirements exist.

The programmer should not need to manually spell out every target-specific implementation detail when that detail can be safely derived.

## Source of truth

The source of truth is a formal semantic program, not generated C, Rust, assembly, or machine code.

Generated implementation artifacts may be inspected, but they are not intended to become independently maintained source.

## Semantics versus presentation

Bister distinguishes between:

1. **presentation** — visual nodes, text, forms, natural language;
2. **semantics** — the normalized meaning represented by the SPG;
3. **implementation** — Bister IR and backend code.

Different authoring interfaces may generate identical semantics.

## Candidate core concepts

The first language model should cover:

### Values

- booleans;
- signed/unsigned integers;
- floating-point values;
- strings only if required by the prototype;
- enums;
- records/structures.

### Units

Physical units should eventually be representable in the type system or constraint system.

Examples:

```text
8 A
100 ms
1 kHz
300 RPM/s
```

Unit-aware semantics are especially valuable for embedded/control applications.

### Variables and state

Bister must clearly distinguish:

- immutable values;
- mutable local state;
- persistent state;
- external state;
- hardware state.

### Control flow

Initial control constructs:

- sequence;
- branch;
- loop;
- function/call;
- state transition;
- termination/failure.

### Effects

Operations with externally visible behavior should declare effects.

Examples:

- memory mutation;
- I/O;
- hardware access;
- file/network access;
- model/tool invocation;
- process creation.

### Constraints

Constraints describe requirements the implementation must satisfy rather than normal procedural steps.

Examples:

```text
current <= 8 A
loop_frequency >= 1 kHz
allocation == static
latency <= 500 us
```

A compiler may reject a target if constraints cannot be proven or satisfied.

### Failure semantics

Failures must be explicit.

A program should be able to specify:

- retry;
- fallback;
- safe state;
- propagation;
- termination;
- recovery conditions.

## Ambiguity

Natural language is inherently ambiguous. Bister must never hide that ambiguity by silently converting every statement into one guessed interpretation.

Possible semantic states include:

- resolved;
- inferred, awaiting approval;
- underspecified;
- conflicting;
- target-dependent.

For example:

> "Stop the motor immediately."

This is insufficient by itself for a hard real-time guarantee. Bister should be able to ask for, or infer and expose, a measurable requirement such as:

```text
motor_disable_latency <= 500 us
```

## AI-generated implementation choices

A semantic requirement may allow several implementations.

Example:

```text
maintain motor speed at target
```

Possible implementations might include different control algorithms.

Bister should distinguish between:

- behavior that the developer specified;
- behavior inferred by AI;
- implementation choices selected by the compiler.

Those categories must remain inspectable.

## Syntax

No surface syntax is standardized yet.

The project should avoid spending early effort debating punctuation before the semantic model is validated.

A future textual format should prioritize:

- readability;
- stable diffs;
- deterministic parsing;
- easy machine generation;
- explicit semantics;
- compatibility with graph-based editing.

## Non-goals for the first prototype

The first prototype does not need:

- object-oriented inheritance;
- macros;
- metaprogramming;
- advanced generics;
- distributed execution;
- a package ecosystem;
- self-hosting;
- garbage collection;
- arbitrary natural-language execution.

The first milestone is to prove that a small semantic program can be normalized, validated, lowered, and executed.
