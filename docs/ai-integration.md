# AI Integration

## Principle

AI is a compiler component, not the language definition.

Bister must remain meaningful if the model provider, model architecture, prompt strategy, or inference runtime changes.

## Appropriate AI responsibilities

AI may assist with:

- translating natural-language requirements into proposed SPG structures;
- expanding high-level semantic operations;
- suggesting algorithms;
- resolving target mappings;
- generating candidate tests;
- explaining compiler errors;
- proposing optimizations;
- identifying missing requirements.

## Inappropriate trust assumptions

A model response must not be assumed correct because:

- it is confident;
- it compiles;
- it resembles conventional code;
- multiple model samples agree.

AI output is untrusted until validated.

## Provider abstraction

The project should eventually define a provider-neutral interface resembling:

```text
reason(request, context, constraints) -> proposal
```

A proposal should be structured and machine-checkable.

Potential providers include:

- local open-weight models;
- remote hosted models;
- specialized code/reasoning models.

## Structured outputs

Whenever possible, models should generate typed structured proposals rather than free-form source.

Example:

```json
{
  "proposal": "add_branch",
  "condition": "motor_current > max_current",
  "on_true": "motor_stop",
  "derived_from": ["constraint:max_current"]
}
```

The compiler then checks whether that transformation is legal.

## Ambiguity protocol

If intent has multiple materially different interpretations, the system should not randomly choose one.

The AI layer should return alternatives or a clarification requirement.

Example:

```text
User: "Stop immediately on overcurrent."

Unresolved:
- required maximum detection latency?
- should shutdown latch?
- is automatic restart permitted?
```

## Privacy

Projects may contain sensitive source, hardware details, credentials, or customer information.

Bister should make model routing explicit:

- local only;
- approved hosted provider;
- no-AI deterministic build.

Secrets should never be included in model context by default.

## Reproducibility

A build should be separable into:

### synthesis phase

May involve nondeterministic model calls.

### accepted semantic artifact

Reviewed/validated representation stored with the project.

### deterministic compile phase

No fresh model call required.

## Model evaluation

Bister should test AI components using semantic tasks rather than subjective chat quality.

Metrics could include:

- valid proposal rate;
- semantic equivalence;
- constraint preservation;
- clarification accuracy;
- regression rate;
- unsafe inference rate.
