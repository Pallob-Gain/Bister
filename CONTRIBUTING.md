# Contributing to Bister

Thank you for helping explore Bister.

Bister is at an early research stage. The most valuable contribution is not necessarily the largest implementation; clear semantics, small prototypes, counterexamples, tests, and well-reasoned design criticism are especially useful.

## Before contributing

Please read:

- [Project architecture](docs/architecture.md)
- [Language design](docs/language-design.md)
- [Semantic Program Graph](docs/semantic-program-graph.md)
- [Bister IR](docs/bister-ir.md)
- [Roadmap](docs/roadmap.md)

## Design-first development

Changes that affect the language, Semantic Program Graph (SPG), Bister IR, verification model, execution model, or AI trust boundary should begin as a GitHub issue.

A design issue should describe:

1. the problem;
2. the proposed semantics;
3. at least one example;
4. alternatives considered;
5. failure cases or ambiguity;
6. compatibility implications;
7. how the proposal can be tested.

Do not treat an LLM-generated answer as a language specification. Specifications must be explicit enough to implement and test independently of a particular model.

## Contribution areas

Useful contribution areas include:

- language semantics;
- compiler front end;
- SPG schema;
- Bister IR;
- static analysis;
- formal verification;
- LLVM/WASM backends;
- embedded targets;
- AI provider adapters;
- visual programming UX;
- diagnostics and explainability;
- examples and tests;
- documentation.

## Suggested labels

We plan to organize design work using labels such as:

- `area:language`
- `area:spg`
- `area:ir`
- `area:compiler`
- `area:ai`
- `area:verification`
- `area:editor`
- `area:embedded`
- `type:design`
- `type:bug`
- `type:experiment`
- `good first issue`

## Pull requests

Keep pull requests focused. A PR should normally contain one coherent change.

Please include:

- what changed;
- why it changed;
- tests or examples;
- relevant design issue;
- known limitations.

For compiler behavior, add tests whenever practical.

## Generated code

Generated source, IR, or binaries should not be committed unless they are intentional fixtures, examples, or golden test artifacts.

## AI-assisted contributions

AI-assisted contributions are welcome, but contributors remain responsible for the correctness, licensing, provenance, and security of submitted work.

Never submit secrets, private code, proprietary specifications, or third-party content without permission.

## Commit style

Descriptive conventional-style prefixes are encouraged:

```
docs: clarify SPG control edges
feat: add integer comparison node
fix: reject untyped state transition
test: cover cyclic graph validation
refactor: separate verifier from lowering
```

## Code of Conduct

Participation is governed by [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## License

By contributing, you agree that your contributions will be licensed under the repository's Apache License 2.0.
