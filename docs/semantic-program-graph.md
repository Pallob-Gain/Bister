# Semantic Program Graph (SPG)

## Status

Draft concept. Not yet a stable specification.

## Purpose

The Semantic Program Graph is Bister's formal source-level representation.

It exists between human-facing authoring tools and compiler IR.

The SPG should preserve meaning that conventional control-flow graphs often omit, such as timing constraints, physical units, resources, failures, and developer intent.

## Design goals

The SPG should be:

- machine readable;
- deterministic to serialize;
- versionable;
- independent of a particular LLM;
- independent of a particular editor;
- rich enough to validate before implementation lowering;
- simple enough to inspect and diff.

## Core entities

### Program

A program owns modules, declarations, entry points, requirements, and metadata.

### Node

A node represents a semantic operation or construct.

Candidate node categories:

- literal;
- input;
- output;
- arithmetic;
- comparison;
- branch;
- loop;
- call;
- state;
- state transition;
- resource operation;
- assertion;
- constraint;
- failure;
- synchronization.

### Port

Typed nodes expose input/output ports.

Ports carry values, events, capabilities, or resources.

### Edge

Different relationships should use different edge kinds rather than overloading a generic connection.

Candidate edge types:

- data edge;
- control edge;
- state edge;
- effect dependency;
- resource dependency;
- constraint reference.

### Constraint

Constraints attach requirements to nodes, values, resources, or the whole program.

### Resource

Resources represent bounded or externally controlled entities such as:

- GPIO pin;
- timer;
- UART;
- memory region;
- network socket;
- thread/task;
- accelerator.

## Illustrative serialized form

This is an example only:

```json
{
  "spg_version": "0.1-draft",
  "program": {
    "name": "FanController",
    "nodes": [
      {
        "id": "temperature",
        "kind": "input",
        "type": { "scalar": "f32", "unit": "degC" }
      },
      {
        "id": "hot",
        "kind": "compare",
        "op": "gt",
        "inputs": ["temperature", "threshold_60"]
      }
    ],
    "constraints": [
      {
        "kind": "range",
        "subject": "fan_speed",
        "min": 0,
        "max": 100,
        "unit": "percent"
      }
    ]
  }
}
```

## Identity

Every semantic entity that may be referenced should have a stable identity within the program.

Editor-specific coordinates, colors, layout, comments, and viewport state should not affect semantic identity.

## Types

The SPG should carry resolved or resolvable types.

A type may eventually include:

- primitive representation;
- unit;
- range;
- ownership/lifetime semantics;
- mutability;
- capability;
- nullability/optional state;
- error domain.

## Control and data flow

Bister should keep control flow and data flow separate.

A data dependency does not necessarily imply sequencing.

This matters for:

- optimization;
- concurrency;
- real-time scheduling;
- hardware pipelines.

## Effects

Effects must be representable so the compiler can reason about ordering and safety.

Example effect sets:

```text
pure
read(sensor.temperature)
write(motor.pwm)
network
filesystem
allocation
model_call
```

## Timing

Timing should be semantic rather than merely documentation.

Possible constructs:

- period;
- deadline;
- maximum latency;
- timeout;
- minimum separation;
- jitter tolerance.

## Failure

Failure paths are part of the program graph.

A hardware read might produce:

```text
value
timeout
device_error
invalid_sample
```

The graph must show which failures are handled and which are propagated.

## AI provenance

AI provenance may be attached as non-semantic metadata:

- model/provider;
- synthesis session;
- transformation identifier;
- confidence or uncertainty notes.

However, program correctness must not depend on retaining a model's hidden reasoning.

## Validation

Before lowering, the SPG validator should detect:

- dangling references;
- invalid edge types;
- type mismatches;
- impossible ranges;
- unresolved required inputs;
- conflicting constraints;
- unhandled required failures;
- cycles where cycles are illegal;
- ambiguous semantic placeholders.

## Open questions

- Should control constructs be explicit nodes or structured regions?
- How much type information belongs in SPG versus Bister IR?
- How are probabilistic requirements represented?
- How should units be encoded?
- How should concurrency be modeled?
- Which constraints must be statically provable versus runtime checked?
