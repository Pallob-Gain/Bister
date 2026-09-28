# Verification Strategy

Bister's central risk is semantic drift: the executable may differ from what the developer intended.

Verification therefore must exist at multiple layers.

## 1. Structural validation

Checks that the SPG itself is well formed:

- valid node kinds;
- valid references;
- legal edges;
- no missing required fields;
- schema compatibility.

## 2. Type and unit validation

Checks:

- type compatibility;
- conversions;
- range correctness;
- unit consistency.

Example:

```text
5 seconds + 2 volts
```

must not silently become valid arithmetic.

## 3. Effect validation

Checks that effectful operations respect ordering, permissions, and capabilities.

Example:

- an operation declared pure may not mutate hardware;
- a restricted target may prohibit filesystem access.

## 4. Constraint validation

Constraints fall into several categories:

### statically provable

Can be proven at compile time.

### target-verifiable

Can be checked after resource allocation or scheduling.

### runtime enforced

Requires generated checks.

### unverified

Cannot currently be proven or enforced.

Bister diagnostics should clearly distinguish these states.

## 5. Control-flow verification

Checks:

- reachability;
- termination where required;
- exhaustive handling;
- invalid cycles;
- impossible transitions.

## 6. Failure-path verification

Critical failures should not disappear during lowering.

The compiler should be able to answer:

- what happens if this operation fails?
- is the failure handled?
- can the safe state itself fail?

## 7. Timing verification

For real-time targets, timing requirements may include:

- deadlines;
- periods;
- latency;
- jitter;
- scheduling conflicts.

Early Bister versions may support only conservative/static subsets.

## 8. Resource verification

Embedded targets require checking:

- pin conflicts;
- peripheral availability;
- timer/channel allocation;
- memory limits;
- DMA resources;
- interrupt constraints.

## 9. Transformation verification

Every lowering/optimization pass should preserve documented invariants.

Model-proposed transformations require the same validation as hand-written compiler passes.

## 10. Tests and simulation

The compiler should be able to derive or accept:

- examples;
- assertions;
- property tests;
- simulation scenarios;
- golden outputs.

AI may generate candidate tests, but tests are not proof by themselves.

## 11. Explainability

For important requirements, the toolchain should provide traceability:

```text
Requirement
   ↓
SPG entity
   ↓
IR checks / generated runtime guard
   ↓
backend implementation
```

Example:

```text
Requirement:
  current <= 8 A

Implementation:
  sample current every 100 us
  compare against threshold
  disable PWM on violation

Verification:
  runtime enforced
  max detection latency: target-dependent
```

## 12. Long-term research

Potential techniques:

- SMT solving;
- symbolic execution;
- model checking;
- abstract interpretation;
- proof-carrying transformations;
- worst-case execution-time analysis;
- equivalence checking.
