# Editor

Bister's future interactive flowchart and semantic-program editor may use standard web technologies such as TypeScript, HTML/CSS, SVG/Canvas, WebAssembly, and an appropriate UI framework.

The editor should provide:

- interactive algorithm/flowchart construction;
- semantic node property editing;
- data/control-flow visualization;
- constraint and timing editing;
- compiler diagnostics;
- AI proposal review;
- SPG diff/review.

## Architectural rule

The editor is a **client** of Bister semantics.

UI coordinates, styling, framework state, and browser-specific data must never become required program semantics.

The editor should communicate with the Rust/C++ compiler through a stable SPG/compiler interface.

The compiler must continue to work without the editor.
