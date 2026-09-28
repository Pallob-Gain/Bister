# Temperature Fan Example

This is the first deliberately small Bister semantic example.

Desired behavior:

```text
temperature > 60 C  -> fan = 100%
temperature > 40 C  -> fan = 50%
otherwise           -> fan = 0%
```

The accompanying `fan-controller.spg.json` is **not a stable format**. It exists to make the first SPG schema discussions concrete.

The example should evolve alongside Phase 1 of the roadmap until it can pass through:

```text
SPG -> validator -> Bister IR -> backend -> executable
```
