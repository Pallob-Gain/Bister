# AI

Future model-provider adapters and semantic synthesis tooling.

Rules:

- AI output is untrusted until validated.
- Provider-specific details must not define Bister semantics.
- Deterministic compilation must remain possible from accepted semantic artifacts without a fresh model call.
- Hosted models must not receive project secrets by default.

See [AI integration](../docs/ai-integration.md).
