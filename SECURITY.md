# Security Policy

Bister is experimental and pre-alpha. It must not currently be relied upon for safety-critical, security-critical, medical, automotive, industrial protection, or other high-consequence production systems.

## Reporting a vulnerability

Please avoid opening a public issue for vulnerabilities that could enable code execution, sandbox escape, supply-chain compromise, malicious model/tool invocation, or unsafe generated binaries.

Use GitHub's private vulnerability reporting feature if enabled. If it is not available, contact the repository owner privately through GitHub.

Include, where possible:

- affected commit or version;
- reproduction steps;
- impact;
- proof of concept;
- suggested mitigation.

## Important threat areas

Bister's architecture introduces security concerns beyond a conventional compiler, including:

- prompt and context injection;
- malicious model output;
- compromised model providers;
- dependency and tool-chain attacks;
- generated unsafe code;
- semantic mismatch between intent and implementation;
- untrusted project metadata;
- target-specific memory unsafety;
- secrets accidentally sent to hosted models.

The compiler must treat AI output as untrusted input until validated.

## Supported versions

There are no stable releases yet. Security fixes apply to the active development branch.
