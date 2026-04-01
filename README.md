# Anvil

Anvil is a terminal-first coding agent and automation CLI for software work. It is built around a Rust engine that owns conversation state, permissions, tool execution, session memory, configuration, and workflow orchestration.

Anvil is meant to ship as its own product. The README should always describe the current Anvil product directly, not its research inputs, not temporary transition language, and not another project's branding.

## Product Snapshot

- Primary binary: `anvil`
- Active product core: `rust/`
- Main runtime crates: `api`, `commands`, `runtime`, `tools`
- Current supported interaction modes: interactive REPL and one-shot prompt execution
- Current platform focus: Windows-first local development with a Rust CLI that also targets cross-platform use
- Current quality signal: the Rust workspace test suite passes end to end

## What Anvil Does Today

Anvil already includes a substantial local product core:

- interactive REPL with slash commands
- one-shot prompt mode
- session persistence, resume, export, and compaction
- model selection and cost reporting
- permission modes and tool allowlists
- local shell and PowerShell execution
- file read, write, and edit tools
- glob and grep search tools
- web fetch and web search tools
- notebook editing and todo tracking
- local skill loading
- sub-agent tool execution primitives
- MCP stdio bootstrap and tool routing
- OAuth and API-key based authentication paths
- project instruction and memory discovery
- usage and pricing tracking

## Current Command Surface

The current REPL command surface includes:

- `/help`
- `/status`
- `/compact`
- `/clear`
- `/model`
- `/permissions`
- `/cost`
- `/resume`
- `/config`
- `/memory`
- `/init`
- `/diff`
- `/version`
- `/export`
- `/session`

## Current Shipping Position

Anvil has moved past bootstrap stage and now has a stable Rust foundation, but it is not yet ready to call feature-complete or ready for a first serious public-quality release.

The current state is:

- product identity is established as Anvil
- the Rust workspace is the active implementation core
- the Rust test suite is green
- core local agent workflows work
- provider abstraction is not finished
- command and tool breadth still needs expansion
- plugins and a full skills registry are still missing
- packaging, CI, release automation, and distribution are still incomplete

The complete remaining-work plan lives in [docs/shipping-plan.md](docs/shipping-plan.md).

## What Must Exist Before Anvil Is Considered Ready

Before we can say the main goal is achieved, Anvil still needs:

- a provider abstraction layer so Anvil is the engine and model vendors are adapters
- at least two production-grade provider adapters, with Anthropic and OpenAI-compatible support as the minimum target
- broader command coverage for planning, review, tasks, MCP, skills, and plugins
- a fuller tool surface across MCP resources, workflow tools, and project operations
- a real skills registry instead of local-file-only loading
- a real plugin system with loading, enablement, and extension hooks
- release-quality packaging, install flow, CI, and smoke-test coverage
- documentation that stays synchronized with every major milestone

## Architecture Summary

Anvil is organized around these layers:

1. Product shell
The CLI, REPL, prompt mode, output formatting, and user-facing command surface.

2. Engine runtime
Conversation state, permissions, session persistence, compaction, prompt assembly, hooks, MCP plumbing, and orchestration.

3. Tool system
Built-in tools, schemas, execution adapters, and policy-aware tool access.

4. Provider adapters
Vendor-specific message streaming and tool-call normalization behind one internal contract.

5. Extension surfaces
Skills, plugins, task orchestration, and future transport layers.

The detailed target structure is in [docs/target-architecture.md](docs/target-architecture.md).

## Repository Layout

```text
.
|-- assets/     Product assets and artwork
|-- docs/       Product docs, roadmap, shipping plan, and coverage analysis
|-- rust/       Active Rust workspace and shipping implementation core
|-- src/        Compatibility and source-analysis surfaces used for coverage work
|-- tests/      Compatibility and porting verification surfaces
|-- PARITY.md   Cross-surface feature coverage assessment
`-- README.md   Product overview and current status
```

## Verification

Run verification from [rust](rust):

```bash
cargo fmt
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Documentation Discipline

After every important milestone:

- update this README with the new current status
- update [docs/shipping-plan.md](docs/shipping-plan.md) if the remaining work changed
- update [docs/roadmap.md](docs/roadmap.md) if phase status changed
- update [docs/target-architecture.md](docs/target-architecture.md) if the implementation shape changed
- update [PARITY.md](PARITY.md) when coverage claims materially change

## Key Documents

- [docs/shipping-plan.md](docs/shipping-plan.md)
- [docs/roadmap.md](docs/roadmap.md)
- [docs/target-architecture.md](docs/target-architecture.md)
- [PARITY.md](PARITY.md)

## Recent Milestones

- bootstrap of the standalone Anvil repository
- product rebrand of the Rust CLI surface to `anvil`
- Windows hardening and full Rust workspace test stabilization
