# Bister Glossary

## Bister

The language/toolchain project as a whole.

## Semantic Program Graph (SPG)

The formal source-level representation of program meaning, including data flow, control flow, state, effects, constraints, failures, timing, and resources.

## Bister IR

The deterministic compiler representation produced after semantic ambiguity is resolved enough for conventional lowering.

## Semantic synthesis

The process of converting human-facing intent into formal semantic structures. This phase may use AI and may be nondeterministic.

## Deterministic build

Compilation from an accepted semantic artifact through validation and backend lowering without requiring a fresh model decision.

## Constraint

A requirement an implementation must satisfy, such as a range, deadline, resource limit, safety condition, or target restriction.

## Effect

An externally observable or state-changing operation, such as I/O, hardware access, allocation, networking, or filesystem access.

## Resource

A bounded or exclusive entity used by a program, such as a timer, pin, DMA channel, thread, peripheral, memory region, or socket.

## Proposal

A structured semantic transformation suggested by an AI component. Proposals are not trusted until validated and accepted.

## Provenance

Metadata describing where a semantic element or transformation came from. Provenance may aid auditing but must not be required to reproduce hidden model reasoning.

## Backend

The deterministic compiler component that translates Bister IR toward a target such as LLVM IR, WebAssembly, or another machine representation.
