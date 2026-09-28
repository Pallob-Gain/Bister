# Bister IR

## Status

Draft architecture.

## Purpose

Bister IR is the deterministic boundary between semantic synthesis and conventional compiler lowering.

The SPG describes the program at a human-semantic level. Bister IR makes execution explicit enough for deterministic validation, optimization, and backend translation.

## Required properties

By the time a program becomes Bister IR:

- control flow is explicit;
- operations have resolved types;
- mutable state is explicit;
- effects are explicit;
- error/failure paths are explicit;
- natural-language statements do not determine execution;
- unresolved semantic ambiguity has been eliminated;
- target assumptions are declared.

## Relationship to LLVM IR

Bister IR should not initially try to replace LLVM.

It should capture concepts that are important to Bister but may disappear during conventional lowering, such as:

- high-level constraints;
- resource identities;
- timing obligations;
- safety obligations;
- effect information;
- provenance from semantic constructs.

Then:

```text
SPG
 ↓
Bister IR
 ↓
constraint/resource lowering
 ↓
LLVM IR
 ↓
machine code
```

## Candidate representation

An initial implementation can use an in-memory typed IR plus a stable textual or serialized debug form.

Illustrative example:

```text
func @control_loop(%temperature: f32<degC>) -> u8<percent> {
entry:
    %hot = cmp.gt %temperature, 60.0<degC>
    br %hot, ^full, ^check_warm

full:
    ret 100<percent>

check_warm:
    %warm = cmp.gt %temperature, 40.0<degC>
    br %warm, ^half, ^off

half:
    ret 50<percent>

off:
    ret 0<percent>
}
```

This syntax is illustrative only.

## IR levels

Bister may eventually benefit from more than one IR level.

### High-level Bister IR

Preserves:

- semantic operations;
- constraints;
- units;
- resource abstractions.

### Lowered Bister IR

Makes explicit:

- memory;
- scheduling;
- target resources;
- error representation;
- runtime calls.

### Backend IR

LLVM IR, WebAssembly, or target-specific form.

## Passes

Candidate pass pipeline:

1. SPG normalization;
2. type and unit resolution;
3. effect resolution;
4. control-flow lowering;
5. failure-path lowering;
6. constraint normalization;
7. resource binding;
8. target legality checks;
9. optimization;
10. backend emission.

## Determinism

No compiler pass after the accepted deterministic boundary should require a fresh model response to produce the same build.

Model-assisted optimization may propose a transformed IR, but that transformed artifact must be captured and revalidated before deterministic compilation continues.

## Verification contract

Every IR instruction and transformation should have documented invariants.

Examples:

- operand types must match;
- control-flow destinations must exist;
- SSA rules, if adopted, must hold;
- resources cannot be used outside declared capabilities;
- runtime checks required by unresolved static constraints must not be discarded.

## Open questions

- SSA versus another representation;
- region-based versus instruction-based structure;
- effect system design;
- memory model;
- concurrency representation;
- exception/error representation;
- unit preservation through optimization.
