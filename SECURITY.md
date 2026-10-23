# Security policy

Rukn is experimental and is not currently suitable for production or for executing untrusted source in privileged services.

## Supported versions

Only the current default branch is under active development. No stable releases or security support window have been declared.

## Reporting a vulnerability

Please avoid public issue reports for exploitable vulnerabilities. Until a dedicated private disclosure address is configured, contact the maintainers through GitHub's private contact channels. Include the affected commit, minimal reproduction, impact and any relevant logs. Do not include secrets or other users' data.

## Current boundaries and limitations

- The language has no file, network or process APIs.
- The CLI reads the source file explicitly supplied by the user.
- The interpreter enforces a 1 MiB UTF-8 source limit, a 16,384-token limit, a parser-nesting limit of 64 levels, and an evaluation-depth limit of 64 levels.
- These limits are defensive guardrails, not a sandbox. They do not isolate the interpreter from the host process or guarantee that every resource-exhaustion path is impossible.
- Deeply nested or oversized input should return a language diagnostic; keep the limits and their regression tests in sync.
- The interpreter is not a security sandbox and should not be embedded as one.
