# Security policy

Rukn is experimental and is not currently suitable for production or for executing untrusted source in privileged services.

## Supported versions

Only the current default branch is under active development. No stable releases or security support window have been declared.

## Reporting a vulnerability

Please avoid public issue reports for exploitable vulnerabilities. Until a dedicated private disclosure address is configured, contact the maintainers through GitHub's private contact channels. Include the affected commit, minimal reproduction, impact and any relevant logs. Do not include secrets or other users' data.

## Current boundaries and limitations

- The language has no file, network or process APIs.
- The CLI reads the source file explicitly supplied by the user.
- Source size, token count, parser nesting and execution limits are not implemented yet.
- Recursive parsing and evaluation can exhaust the process stack on deeply nested input.
- The interpreter is not a security sandbox and should not be embedded as one.
